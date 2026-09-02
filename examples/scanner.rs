//! Read-only opportunity scanner for Kalshi.
//!
//! Two independent scans, both purely observational - this program never
//! places, amends, or cancels an order:
//!
//! 1. **Spreads** across the crypto 15-minute family. Maker fees are zero on
//!    standard markets, so the quoted spread is the gross capture per round
//!    trip. The interesting markets are the wide, thinly contested ones, not
//!    the tightest.
//!
//! 2. **Multivariate (MVE) parlay combos** against the legs they are built
//!    from. A parlay pays only if every leg hits, which gives two model-free
//!    constraints that hold under *any* correlation structure:
//!
//!    - upper: `P(combo) <= min_i P(leg_i)`
//!    - lower: `P(combo) >= max(0, sum_i P(leg_i) - (n - 1))`   (Fréchet)
//!
//!    Comparing the quoted ask against those bounds, and against the
//!    independence price `prod_i P(leg_i)`, measures how much the combo is
//!    marked up over what its legs imply.
//!
//! Usage:
//!   cargo run --example scanner                 # one pass, both scans
//!   cargo run --example scanner -- spreads      # spreads only
//!   cargo run --example scanner -- parlays      # parlays only
//!   cargo run --example scanner -- --watch 30   # repeat every 30s
//!
//! Requires KALSHI_API_KEY_ID and KALSHI_PK_FILE_PATH.

use kalshi_rs::auth::auth_loader::load_auth_from_file;
use kalshi_rs::markets::models::{Market, MarketsQuery};
use kalshi_rs::{KalshiClient, RateLimitTier};
use std::collections::HashMap;
use std::time::Duration;

/// The crypto 15-minute family. Same contract structure, wildly different
/// levels of competition.
const CRYPTO_15M: [&str; 6] = [
    "KXBTC15M",
    "KXETH15M",
    "KXSOL15M",
    "KXDOGE15M",
    "KXBNB15M",
    "KXXRP15M",
];

/// Skip combos with more legs than this: each extra leg widens the bounds
/// until they say nothing useful, and costs another leg to price.
const MAX_LEGS: usize = 4;

/// Kalshi's taker fee, in dollars per contract, at price `p`.
///
/// `0.07 * p * (1 - p)`, peaking at 1.75c near 50c. Makers pay zero on
/// standard markets, which is why the spread scan reports gross spread.
fn taker_fee(p: f64) -> f64 {
    0.07 * p * (1.0 - p)
}

/// A quote at the extremes is the "nobody is here" state rather than a real
/// price: an ask of 1.0000 means no one is offering, a bid of 0.0000 means no
/// one is bidding. Treating those as tradeable manufactures fake edge.
const QUOTE_FLOOR: f64 = 0.005;
const QUOTE_CEIL: f64 = 0.995;

fn parse_price(s: &str) -> Option<f64> {
    let v = s.parse::<f64>().ok()?;
    (v > QUOTE_FLOOR && v < QUOTE_CEIL).then_some(v)
}

/// Best bid / best ask for the yes side, when both sides are genuinely quoted.
fn yes_quote(m: &Market) -> Option<(f64, f64)> {
    let bid = parse_price(&m.yes_bid_dollars)?;
    let ask = parse_price(&m.yes_ask_dollars)?;
    (ask > bid).then_some((bid, ask))
}

/// Minimum price increment at `price`, read from the market's own
/// `price_ranges`.
///
/// Kalshi uses a tapered structure: 0.1c steps in the tails, 1c between $0.10
/// and $0.90. Raw spreads are therefore not comparable across markets sitting
/// at different price levels - a 1c spread is the tightest possible quote
/// mid-book but a 10-tick spread in the tail. Spread-in-ticks is the
/// comparable measure.
fn tick_at(m: &Market, price: f64) -> Option<f64> {
    m.price_ranges
        .iter()
        .find_map(|r| {
            let start = r.start.parse::<f64>().ok()?;
            let end = r.end.parse::<f64>().ok()?;
            (price >= start && price <= end).then(|| r.step.parse::<f64>().ok())?
        })
        .filter(|s| *s > 0.0)
}

// ---------------------------------------------------------------- spreads

struct SpreadRow {
    series: String,
    #[allow(dead_code)]
    ticker: String,
    bid: f64,
    ask: f64,
    volume: f64,
    /// Resting size at the best bid, i.e. the queue you join.
    queue: f64,
    /// Minimum increment at this price level.
    tick: f64,
}

impl SpreadRow {
    fn spread(&self) -> f64 {
        self.ask - self.bid
    }
    /// Spread measured in ticks - comparable across price levels, unlike raw
    /// cents. A value of 1 is the tightest quote the market permits.
    fn ticks(&self) -> f64 {
        self.spread() / self.tick
    }
}

async fn scan_spreads(client: &KalshiClient) -> Vec<SpreadRow> {
    let mut rows = Vec::new();
    for series in CRYPTO_15M {
        let markets = match client
            .get_all_markets(&MarketsQuery {
                limit: Some(100),
                series_ticker: Some(series.to_string()),
                status: Some("open".to_string()),
                ..Default::default()
            })
            .await
        {
            Ok(m) => m.markets,
            Err(e) => {
                eprintln!("  {series}: {e}");
                continue;
            }
        };

        for m in markets {
            let Some((bid, ask)) = yes_quote(&m) else {
                continue;
            };
            // Depth at the touch tells you how much size is ahead of you in
            // the queue, which drives fill rate far more than spread does.
            let queue = client
                .get_market_orderbook(&m.ticker, Some(5))
                .await
                .ok()
                .and_then(|ob| {
                    let yes = ob.orderbook_fp.yes_dollars?;
                    // Levels come back ascending, so the best bid is last.
                    yes.last().and_then(|(_, sz)| sz.parse::<f64>().ok())
                })
                .unwrap_or(f64::NAN);

            let tick = tick_at(&m, bid).unwrap_or(0.01);
            rows.push(SpreadRow {
                series: series.to_string(),
                ticker: m.ticker.clone(),
                bid,
                ask,
                volume: m.volume_fp.parse().unwrap_or(0.0),
                queue,
                tick,
            });
        }
    }
    // Widest in ticks first: that is where the maker capture is largest
    // relative to what the market's own price grid allows.
    rows.sort_by(|a, b| b.ticks().partial_cmp(&a.ticks()).unwrap_or(std::cmp::Ordering::Equal));
    rows
}

fn report_spreads(rows: &[SpreadRow]) {
    println!("\n=== CRYPTO 15M SPREADS (maker fee = 0, so spread = gross capture) ===");
    if rows.is_empty() {
        println!("  no genuinely quoted markets right now");
        return;
    }
    println!(
        "{:<11} {:>7} {:>7} {:>8} {:>6} {:>7} {:>8} {:>10}",
        "series", "bid", "ask", "spread", "tick", "ticks", "queue", "volume"
    );
    for r in rows {
        println!(
            "{:<11} {:>7.4} {:>7.4} {:>7.2}c {:>5.2}c {:>6.1}x {:>8.0} {:>10.0}",
            r.series,
            r.bid,
            r.ask,
            r.spread() * 100.0,
            r.tick * 100.0,
            r.ticks(),
            r.queue,
            r.volume
        );
    }
    println!(
        "\n  'ticks' is spread / minimum increment at that price - the comparable\n  \
         measure, since Kalshi's tick is 0.1c in the tails and 1c from $0.10-$0.90.\n  \
         1.0x means the quote is already as tight as the grid allows.\n  \
         A taker round trip at mid costs ~{:.2}c in fees alone."
        ,
        2.0 * taker_fee(0.5) * 100.0
    );
}

// ---------------------------------------------------------------- parlays

struct ParlayRow {
    ticker: String,
    ask: f64,
    legs: usize,
    /// Product of leg probabilities: the price if legs were independent.
    independence: f64,
    /// `min_i P(leg_i)` - a parlay cannot beat its weakest leg.
    upper_bound: f64,
    /// Fréchet lower bound.
    lower_bound: f64,
    /// Widest leg spread, the uncertainty band on every number above.
    worst_leg_spread: f64,
}

impl ParlayRow {
    fn markup(&self) -> f64 {
        if self.independence > 0.0 {
            self.ask / self.independence
        } else {
            f64::NAN
        }
    }
    /// Flag only when the ask clears the ceiling by more than leg-pricing
    /// noise. Without this guard, ordinary wide leg spreads masquerade as
    /// arbitrage.
    fn violates_upper(&self) -> bool {
        self.ask > self.upper_bound + self.worst_leg_spread
    }
}

async fn scan_parlays(client: &KalshiClient) -> Vec<ParlayRow> {
    let markets = match client
        .get_all_markets(&MarketsQuery {
            limit: Some(400),
            status: Some("open".to_string()),
            ..Default::default()
        })
        .await
    {
        Ok(m) => m.markets,
        Err(e) => {
            eprintln!("  listing markets: {e}");
            return Vec::new();
        }
    };

    // Only combos that are actually quoted and small enough to price.
    let combos: Vec<&Market> = markets
        .iter()
        .filter(|m| {
            m.mve_selected_legs
                .as_ref()
                .is_some_and(|l| !l.is_empty() && l.len() <= MAX_LEGS)
                && parse_price(&m.yes_ask_dollars).is_some()
        })
        .collect();

    println!(
        "  {} open markets, {} quoted combos with <= {} legs",
        markets.len(),
        combos.len(),
        MAX_LEGS
    );

    // Price every distinct leg in one batched request rather than one call
    // per leg - the same leg recurs across many combos.
    let mut wanted: Vec<String> = combos
        .iter()
        .flat_map(|c| c.mve_selected_legs.as_ref().unwrap())
        .map(|l| l.market_ticker.clone())
        .collect();
    wanted.sort();
    wanted.dedup();

    let mut leg_prices: HashMap<String, (f64, f64)> = HashMap::new();
    for chunk in wanted.chunks(20) {
        match client
            .get_all_markets(&MarketsQuery {
                limit: Some(100),
                tickers: Some(chunk.join(",")),
                ..Default::default()
            })
            .await
        {
            Ok(resp) => {
                for m in resp.markets {
                    if let Some(q) = yes_quote(&m) {
                        leg_prices.insert(m.ticker.clone(), q);
                    }
                }
            }
            Err(e) => eprintln!("  leg batch failed: {e}"),
        }
    }
    println!("  priced {}/{} distinct legs", leg_prices.len(), wanted.len());

    let mut rows = Vec::new();
    for c in combos {
        let legs = c.mve_selected_legs.as_ref().unwrap();
        let ask = parse_price(&c.yes_ask_dollars).unwrap();

        let mut independence = 1.0f64;
        let mut upper = 1.0f64;
        let mut sum = 0.0f64;
        let mut worst_spread = 0.0f64;
        let mut complete = true;

        for l in legs {
            let Some(&(bid, lask)) = leg_prices.get(&l.market_ticker) else {
                complete = false;
                break;
            };
            let mid = (bid + lask) / 2.0;
            // A "no" leg hits when the underlying market does not.
            let p = if l.side == "yes" { mid } else { 1.0 - mid };
            independence *= p;
            upper = upper.min(p);
            sum += p;
            worst_spread = worst_spread.max(lask - bid);
        }
        if !complete {
            continue;
        }

        rows.push(ParlayRow {
            ticker: c.ticker.clone(),
            ask,
            legs: legs.len(),
            independence,
            upper_bound: upper,
            lower_bound: (sum - (legs.len() as f64 - 1.0)).max(0.0),
            worst_leg_spread: worst_spread,
        });
    }
    rows.sort_by(|a, b| b.markup().partial_cmp(&a.markup()).unwrap_or(std::cmp::Ordering::Equal));
    rows
}

fn report_parlays(rows: &[ParlayRow]) {
    println!("\n=== MVE PARLAY COMBOS vs THEIR LEGS ===");
    if rows.is_empty() {
        println!("  no priceable quoted combos this pass");
        return;
    }
    println!(
        "{:>4} {:>8} {:>10} {:>8} {:>8} {:>7} {:>6}  {}",
        "legs", "ask", "indep", "upper", "lower", "markup", "flag", "ticker"
    );
    for r in rows.iter().take(25) {
        println!(
            "{:>4} {:>8.4} {:>10.4} {:>8.4} {:>8.4} {:>6.2}x {:>6}  {}",
            r.legs,
            r.ask,
            r.independence,
            r.upper_bound,
            r.lower_bound,
            r.markup(),
            if r.violates_upper() { "OVER" } else { "" },
            &r.ticker[..r.ticker.len().min(46)]
        );
    }

    let markups: Vec<f64> = rows.iter().map(|r| r.markup()).filter(|m| m.is_finite()).collect();
    if !markups.is_empty() {
        let mut sorted = markups.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = sorted[sorted.len() / 2];
        let over = rows.iter().filter(|r| r.violates_upper()).count();
        println!(
            "\n  {} combos priced | median markup over independence: {:.2}x | \
             {} above the model-free ceiling",
            rows.len(),
            median,
            over
        );
        println!(
            "  Markup > 1 means the ask exceeds what independent legs imply. Correlated\n  \
             legs justify some of that; 'OVER' rows exceed the ceiling for ANY correlation."
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let watch = args
        .iter()
        .position(|a| a == "--watch")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse::<u64>().ok());
    let mode = args
        .iter()
        .find(|a| !a.starts_with("--") && a.parse::<u64>().is_err())
        .cloned()
        .unwrap_or_else(|| "all".into());

    let account = load_auth_from_file()?;
    // Pace ourselves: the parlay scan issues a lot of reads in a burst.
    let client = KalshiClient::new(account).with_rate_limit(RateLimitTier::Basic);

    loop {
        println!("\n{}", "=".repeat(78));
        println!("KALSHI SCANNER (read-only)  mode={mode}");
        println!("{}", "=".repeat(78));

        if mode == "all" || mode == "spreads" {
            let rows = scan_spreads(&client).await;
            report_spreads(&rows);
        }
        if mode == "all" || mode == "parlays" {
            println!("\nscanning parlays...");
            let rows = scan_parlays(&client).await;
            report_parlays(&rows);
        }

        match watch {
            Some(secs) => {
                println!("\n(sleeping {secs}s; ctrl-c to stop)");
                tokio::time::sleep(Duration::from_secs(secs)).await;
            }
            None => break,
        }
    }
    Ok(())
}
