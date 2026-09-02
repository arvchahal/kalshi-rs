use crate::common::{active_market_ticker, setup_ws_client};
use kalshi_rs::websocket::models::*;
use std::time::Duration;
use tokio::time::{timeout, Instant};

/// How long to wait for market traffic before treating the market as quiet.
const LISTEN_BUDGET: Duration = Duration::from_secs(45);
/// Cap on a single `next_message` wait; Kalshi pings well inside this.
const MESSAGE_TIMEOUT: Duration = Duration::from_secs(20);

/// Outcome of listening to a channel for a while.
struct Listen {
    /// Whether the subscription was acknowledged.
    subscribed: bool,
    /// Whether at least one payload message of the expected type arrived.
    saw_payload: bool,
    /// Whether an orderbook snapshot arrived, which opens every
    /// `orderbook_delta` subscription.
    saw_snapshot: bool,
}

/// Subscribe to `channel` for `ticker` and watch until `want` matches a
/// message or the listen budget runs out.
///
/// Fails on anything that indicates a real SDK or API problem - a message the
/// SDK could not parse, or an error frame from Kalshi. Heartbeats and
/// subscription acks are expected traffic, not failures. A market that simply
/// stays quiet is reported rather than failed: no trade printing in 45s says
/// nothing about whether the SDK parses trades correctly.
async fn listen(
    channel: &str,
    ticker: &str,
    want: impl Fn(&KalshiSocketMessage) -> bool,
) -> Listen {
    let client = setup_ws_client();
    client.connect().await.expect("ws connect failed");
    client
        .subscribe(vec![channel], vec![ticker])
        .await
        .expect("ws subscribe failed");

    let mut out = Listen {
        subscribed: false,
        saw_payload: false,
        saw_snapshot: false,
    };
    let deadline = Instant::now() + LISTEN_BUDGET;

    while Instant::now() < deadline {
        let next = timeout(MESSAGE_TIMEOUT, client.next_message()).await;
        let message = match next {
            Err(_) => break, // no traffic at all within the per-message budget
            Ok(m) => m.expect("ws stream error"),
        };

        match &message {
            KalshiSocketMessage::Unparseable(raw) => {
                panic!("SDK could not parse a `{channel}` message: {raw}");
            }
            KalshiSocketMessage::ErrorResponse(e) => {
                panic!("Kalshi returned an error on `{channel}`: {e:?}");
            }
            KalshiSocketMessage::SubscribedResponse(_) => out.subscribed = true,
            KalshiSocketMessage::OrderbookSnapshot(_) => out.saw_snapshot = true,
            _ => {}
        }

        if want(&message) {
            out.saw_payload = true;
            break;
        }
    }
    out
}

#[tokio::test]
async fn test_orderbook_delta() {
    let ticker = active_market_ticker().await;
    println!("orderbook_delta on {ticker}");
    // Wait specifically for a delta rather than stopping at the snapshot that
    // opens the subscription - otherwise the delta model is never exercised.
    let result = listen("orderbook_delta", &ticker, |m| {
        matches!(m, KalshiSocketMessage::OrderbookDelta(_))
    })
    .await;
    assert!(result.subscribed, "no subscription ack for orderbook_delta");
    // Every orderbook_delta subscription opens with a snapshot, so this one
    // is safe to require outright.
    assert!(
        result.saw_snapshot,
        "no orderbook snapshot within {LISTEN_BUDGET:?}"
    );
    if !result.saw_payload {
        println!("No orderbook delta on {ticker} within {LISTEN_BUDGET:?} - book was static.");
    }
}

#[tokio::test]
async fn test_trade_update() {
    let ticker = active_market_ticker().await;
    println!("trade on {ticker}");
    let result = listen("trade", &ticker, |m| {
        matches!(m, KalshiSocketMessage::TradeUpdate(_))
    })
    .await;
    assert!(result.subscribed, "no subscription ack for trade");
    if !result.saw_payload {
        println!("No trade printed on {ticker} within {LISTEN_BUDGET:?} - market was quiet.");
    }
}

#[tokio::test]
async fn test_ticker_update() {
    let ticker = active_market_ticker().await;
    println!("ticker on {ticker}");
    let result = listen("ticker", &ticker, |m| {
        matches!(m, KalshiSocketMessage::TickerUpdate(_))
    })
    .await;
    assert!(result.subscribed, "no subscription ack for ticker");
    if !result.saw_payload {
        println!("No ticker update on {ticker} within {LISTEN_BUDGET:?} - market was quiet.");
    }
}
