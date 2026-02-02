// for rest tests
pub const LIVE_SERIES_TICKER: &'static str = "KXBTC";
pub const LIVE_EVENT_TICKER: &'static str = "KXBTC-26FEB0617-T70250";


// for websocket tests
pub const TEST_MARKET_TICKER: &str = "KXBTCD-26FEB0117-T77249.99";
pub const TEST_ADD_MARKET_TICKER: &str = "KXBTCD-26FEB0117-T77749.99";
pub const CHANNELS: [&str; 5] = [
    "orderbook_delta",
    "trade",
    "fill",
    "ticker",
    "market_positions",
];
