use kalshi_rs::auth::auth_loader::load_auth_from_file;
use kalshi_rs::rate_limit::{RateLimitTier, RateLimiter};
use kalshi_rs::KalshiClient;
use kalshi_rs::KalshiWebsocketClient;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

/// One limiter shared by every client in the test binary.
///
/// The suite runs tests concurrently, so each test having its own budget
/// would still let the process as a whole blow past Kalshi's limit and take
/// 429s. Sharing a single limiter keeps the whole run inside one budget.
fn shared_limiter() -> Arc<RateLimiter> {
    static LIMITER: OnceLock<Arc<RateLimiter>> = OnceLock::new();
    LIMITER
        .get_or_init(|| Arc::new(RateLimiter::new(RateLimitTier::Basic)))
        .clone()
}

pub fn setup_client() -> KalshiClient {
    let account = load_auth_from_file().expect("Failed to load auth credentials");
    KalshiClient::new(account).with_rate_limiter(shared_limiter())
}

pub fn setup_ws_client() -> KalshiWebsocketClient {
    let account = load_auth_from_file().expect("Failed to load auth credentials");
    KalshiWebsocketClient::new(account)
}

/// Ticker of a market that is trading right now.
///
/// Tests must not hard-code tickers: Kalshi markets expire, and a settled
/// market emits no orderbook, trade, or ticker traffic, which makes any test
/// subscribed to it hang or fail for reasons unrelated to the SDK.
///
/// Picks the most frequently traded ticker out of the recent exchange-wide
/// trade feed. `GET /markets?status=open` is not a good source here: it is
/// dominated by generated multivariate-collection markets that carry no
/// volume and produce no websocket traffic.
pub async fn active_market_ticker() -> String {
    let client = setup_client();
    let trades = client
        .get_trades(Some(100), None, None, None, None)
        .await
        .expect("failed to fetch recent trades");

    let mut counts: HashMap<String, usize> = HashMap::new();
    for t in &trades.trades {
        *counts.entry(t.ticker.clone()).or_default() += 1;
    }
    let (ticker, _) = counts
        .into_iter()
        .max_by_key(|(_, n)| *n)
        .expect("no recent trades on the exchange");
    ticker
}
