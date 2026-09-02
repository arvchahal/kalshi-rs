//! Read-only adverse-selection ("markout") logger.
//!
//! The scanner reports quoted spreads, but a spread you never capture is
//! worthless, and one you capture just before the market runs you over is
//! worse than worthless. This measures the part that matters.
//!
//! Every trade print has a maker on the other side. If the taker bought yes,
//! some maker sold yes at that price; if the taker sold yes, a maker bought.
//! We reconstruct that maker's position and ask what happened to the mid a
//! few seconds later:
//!
//! ```text
//!   captured edge = |fill - mid_at_fill|         the half-spread earned
//!   markout(h)    = (mid_{t+h} - fill)  if the maker ended up long yes
//!                   (fill - mid_{t+h})  if the maker ended up short yes
//! ```
//!
//! A market maker is profitable when markout stays positive. If markout is
//! negative while captured edge is positive, the spread is an illusion: you
//! are being picked off by informed flow faster than you earn.
//!
//! This program only subscribes and listens. It never places an order.
//!
//! Usage:
//!   cargo run --example markout                 # 5 minutes, default series
//!   cargo run --example markout -- --secs 900

use kalshi_rs::auth::auth_loader::load_auth_from_file;
use kalshi_rs::markets::models::MarketsQuery;
use kalshi_rs::websocket::models::KalshiSocketMessage;
use kalshi_rs::{KalshiClient, KalshiWebsocketClient, RateLimitTier};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::time::Duration;
use tokio::time::{timeout, Instant};

/// BTC is the control: it is the most contested book on the exchange, so its
/// markout is the benchmark everything else has to beat.
const DEFAULT_SERIES: &str = "KXBTC15M,KXETH15M,KXSOL15M,KXBNB15M,KXXRP15M,KXDOGE15M";

/// Horizons to measure the mid against, in seconds.
const HORIZONS: [f64; 3] = [1.0, 5.0, 15.0];

/// Prices are carried as integer ten-thousandths of a dollar so book keys are
/// exact; Kalshi's finest tick is 0.001 (a tenth of a cent).
type Px = i64;

fn to_px(s: &str) -> Option<Px> {
    Some((s.parse::<f64>().ok()? * 10_000.0).round() as Px)
}

/// Top of book for one market, rebuilt from a snapshot plus deltas.
///
/// Kalshi publishes two bid ladders rather than a bid and an ask: `yes` holds
/// bids to buy yes, `no` holds bids to buy no. The yes ask is therefore
/// `1 - best_no_bid`.
#[derive(Default)]
struct Book {
    yes: BTreeMap<Px, f64>,
    no: BTreeMap<Px, f64>,
}

impl Book {
    fn apply_snapshot(&mut self, yes: &[(String, String)], no: &[(String, String)]) {
        self.yes.clear();
        self.no.clear();
        for (p, s) in yes {
            if let (Some(p), Ok(s)) = (to_px(p), s.parse::<f64>()) {
                if s > 0.0 {
                    self.yes.insert(p, s);
                }
            }
        }
        for (p, s) in no {
            if let (Some(p), Ok(s)) = (to_px(p), s.parse::<f64>()) {
                if s > 0.0 {
                    self.no.insert(p, s);
                }
            }
        }
    }

    fn apply_delta(&mut self, side: &str, price: Px, delta: f64) {
        let level = if side == "yes" {
            &mut self.yes
        } else {
            &mut self.no
        };
        let size = level.entry(price).or_insert(0.0);
        *size += delta;
        if *size <= 0.0 {
            level.remove(&price);
        }
    }

    /// Mid of the yes market in dollars, when both sides are populated.
    fn mid(&self) -> Option<f64> {
        let best_yes_bid = *self.yes.keys().next_back()?;
        let best_no_bid = *self.no.keys().next_back()?;
        let best_yes_ask = 10_000 - best_no_bid;
        (best_yes_ask > best_yes_bid).then(|| (best_yes_bid + best_yes_ask) as f64 / 2.0 / 10_000.0)
    }
}

/// A reconstructed maker fill awaiting evaluation at a future horizon.
struct Pending {
    series: String,
    ticker: String,
    at: Instant,
    /// Yes-price the trade printed at, in dollars.
    fill: f64,
    /// True when the maker ended up long yes (i.e. the taker sold yes).
    maker_long_yes: bool,
    mid_at_fill: f64,
}

#[derive(Default)]
struct Stats {
    fills: usize,
    /// Half-spread the maker earned at the moment of the fill.
    edge: f64,
    /// Realised markout at each horizon, and how many were evaluable.
    markout: [f64; HORIZONS.len()],
    /// Sum of squares, for the standard error of the mean.
    markout_sq: [f64; HORIZONS.len()],
    counted: [usize; HORIZONS.len()],
}

impl Stats {
    /// Mean markout and its standard error, in cents per contract.
    ///
    /// The standard error is deliberately prominent: per-fill markout variance
    /// is on the order of a tick, so a few hundred fills leaves an error bar
    /// wide enough to swallow the mean. Reading a point estimate without it
    /// produces confident nonsense.
    fn markout_ci(&self, i: usize) -> Option<(f64, f64)> {
        let n = self.counted[i];
        if n < 2 {
            return None;
        }
        let nf = n as f64;
        let mean = self.markout[i] / nf;
        let var = ((self.markout_sq[i] / nf) - mean * mean).max(0.0);
        Some((mean * 100.0, (var / nf).sqrt() * 100.0))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let run_secs: u64 = args
        .iter()
        .position(|a| a == "--secs")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(300);

    // Comma-separated series to watch, so the tool can be pointed at sports,
    // metals, or anything else without a rebuild.
    let series_arg = args
        .iter()
        .position(|a| a == "--series")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| DEFAULT_SERIES.to_string());
    let series_list: Vec<&str> = series_arg.split(',').map(|s| s.trim()).collect();

    // Markets per series: sports series carry many simultaneous contracts,
    // and one alone may never trade.
    let per_series: usize = args
        .iter()
        .position(|a| a == "--per-series")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    let account = load_auth_from_file()?;
    let rest = KalshiClient::new(account).with_rate_limit(RateLimitTier::Basic);

    // Resolve live markets per series.
    let mut tickers: Vec<String> = Vec::new();
    let mut series_of: HashMap<String, String> = HashMap::new();
    for s in series_list {
        match rest
            .get_all_markets(&MarketsQuery {
                limit: Some(60),
                series_ticker: Some(s.to_string()),
                status: Some("open".to_string()),
                ..Default::default()
            })
            .await
        {
            Ok(r) => {
                // Prefer markets that are genuinely two-sided; an unquoted
                // contract contributes nothing but a subscription slot.
                let mut quoted: Vec<_> = r
                    .markets
                    .iter()
                    .filter(|m| {
                        let b = m.yes_bid_dollars.parse::<f64>().unwrap_or(0.0);
                        let a = m.yes_ask_dollars.parse::<f64>().unwrap_or(1.0);
                        b > 0.005 && a < 0.995 && a > b
                    })
                    .collect();
                quoted.sort_by(|a, b| {
                    let v = |m: &kalshi_rs::markets::models::Market| {
                        m.volume_fp.parse::<f64>().unwrap_or(0.0)
                    };
                    v(b).partial_cmp(&v(a)).unwrap_or(std::cmp::Ordering::Equal)
                });
                for m in quoted.into_iter().take(per_series) {
                    series_of.insert(m.ticker.clone(), s.to_string());
                    tickers.push(m.ticker.clone());
                }
            }
            Err(e) => eprintln!("  {s}: {e}"),
        }
    }
    if tickers.is_empty() {
        eprintln!("no live markets resolved");
        return Ok(());
    }
    println!("watching {} markets for {run_secs}s:", tickers.len());
    for t in &tickers {
        println!("   {} ({})", t, series_of[t]);
    }

    let ws_account = load_auth_from_file()?;
    let ws = KalshiWebsocketClient::new(ws_account);
    ws.connect().await?;
    let refs: Vec<&str> = tickers.iter().map(|s| s.as_str()).collect();
    ws.subscribe(vec!["orderbook_delta", "trade"], refs).await?;

    let mut books: HashMap<String, Book> = HashMap::new();
    let mut stats: HashMap<String, Stats> = HashMap::new();
    // One queue per horizon; each stays in time order, so the front is always
    // the next one due.
    let mut queues: Vec<VecDeque<Pending>> = HORIZONS.iter().map(|_| VecDeque::new()).collect();

    let deadline = Instant::now() + Duration::from_secs(run_secs);
    let mut trades = 0usize;

    while Instant::now() < deadline {
        // Short poll so pending markouts still resolve when the tape is quiet.
        if let Ok(Ok(message)) =
            timeout(Duration::from_millis(250), ws.next_message()).await
        {
            match &message {
                KalshiSocketMessage::OrderbookSnapshot(s) => {
                    let m = &s.msg;
                    books
                        .entry(m.market_ticker.clone())
                        .or_default()
                        .apply_snapshot(
                            m.yes_dollars.as_deref().unwrap_or(&[]),
                            m.no_dollars.as_deref().unwrap_or(&[]),
                        );
                }
                KalshiSocketMessage::OrderbookDelta(d) => {
                    let m = &d.msg;
                    if let (Some(px), Ok(delta)) = (to_px(&m.price_dollars), m.delta_fp.parse::<f64>())
                    {
                        books
                            .entry(m.market_ticker.clone())
                            .or_default()
                            .apply_delta(&m.side, px, delta);
                    }
                }
                KalshiSocketMessage::TradeUpdate(t) => {
                    let m = &t.msg;
                    let (Some(book), Ok(fill)) = (
                        books.get(&m.market_ticker),
                        m.yes_price_dollars.parse::<f64>(),
                    ) else {
                        continue;
                    };
                    let Some(mid) = book.mid() else { continue };
                    let Some(series) = series_of.get(&m.market_ticker) else {
                        continue;
                    };
                    // taker_side == "yes" means the taker lifted the offer, so
                    // the maker sold yes and is short.
                    let maker_long_yes = m.taker_side != "yes";
                    trades += 1;

                    let st = stats.entry(series.clone()).or_default();
                    st.fills += 1;
                    st.edge += (fill - mid).abs();

                    for q in queues.iter_mut() {
                        q.push_back(Pending {
                            series: series.clone(),
                            ticker: m.market_ticker.clone(),
                            at: Instant::now(),
                            fill,
                            maker_long_yes,
                            mid_at_fill: mid,
                        });
                    }
                }
                KalshiSocketMessage::Unparseable(raw) => {
                    eprintln!("unparseable: {}", &raw[..raw.len().min(160)]);
                }
                _ => {}
            }
        }

        // Resolve everything that has aged past its horizon.
        let now = Instant::now();
        for (hi, horizon) in HORIZONS.iter().enumerate() {
            while queues[hi]
                .front()
                .is_some_and(|p| now.duration_since(p.at).as_secs_f64() >= *horizon)
            {
                let p = queues[hi].pop_front().unwrap();
                // Find the market for this series to read its current mid.
                // Evaluate against the same market the fill came from, not
                // merely the same series - with several contracts per series
                // those mids are unrelated.
                let mid_now = books.get(&p.ticker).and_then(|b| b.mid());
                let Some(mid_now) = mid_now else { continue };
                let pnl = if p.maker_long_yes {
                    mid_now - p.fill
                } else {
                    p.fill - mid_now
                };
                let st = stats.entry(p.series.clone()).or_default();
                st.markout[hi] += pnl;
                st.markout_sq[hi] += pnl * pnl;
                st.counted[hi] += 1;
                let _ = p.mid_at_fill;
            }
        }
    }

    // ------------------------------------------------------------- report
    println!("\n{}", "=".repeat(78));
    println!("MAKER MARKOUT  ({trades} trade prints observed)");
    println!("{}", "=".repeat(78));
    println!(
        "{:<11} {:>6} {:>9}   {:>16} {:>16} {:>16}",
        "series", "fills", "edge", "mk 1s", "mk 5s", "mk 15s"
    );
    let mut rows: Vec<(&String, &Stats)> = stats.iter().collect();
    rows.sort_by_key(|(s, _)| s.as_str());
    for (series, st) in rows {
        if st.fills == 0 {
            continue;
        }
        let edge = st.edge / st.fills as f64 * 100.0;
        let cells: Vec<String> = (0..HORIZONS.len())
            .map(|i| match st.markout_ci(i) {
                None => "-".into(),
                // A mean smaller than its own error bar is indistinguishable
                // from zero; mark it so it cannot be read as a result.
                Some((mean, se)) => {
                    let flag = if mean.abs() < se { "?" } else { " " };
                    format!("{mean:+.2}+-{se:.2}c{flag}")
                }
            })
            .collect();
        println!(
            "{:<11} {:>6} {:>8.3}c   {:>16} {:>16} {:>16}",
            series, st.fills, edge, cells[0], cells[1], cells[2]
        );
    }
    println!(
        "\n  edge  = half-spread the maker earned at the fill (cents/contract)\n  \
         mk Ns = maker P&L N seconds later, same units. Maker fees are zero on\n  \
         standard markets, so these are net.\n  \
         Negative markout means informed flow is taking more than the spread pays.\n  \
         '?' marks a mean smaller than its own standard error - consistent with zero.\n  \
         Note fills cluster in bursts driven by one underlying move, so they are not\n  \
         independent samples: the true error is WIDER than shown. Run for hours."
    );
    Ok(())
}
