use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub ticker: String,
    pub event_ticker: String,
    pub market_type: String,
    pub title: String,
    #[serde(default)]
    pub subtitle: String,
    pub yes_sub_title: String,
    pub no_sub_title: String,
    pub open_time: String,
    pub close_time: String,
    #[serde(default)]
    pub expected_expiration_time: Option<String>,
    #[serde(default)]
    pub expiration_time: Option<String>,
    pub latest_expiration_time: String,
    pub settlement_timer_seconds: u32,
    pub status: String,
    pub response_price_units: String,
    pub notional_value: u32,
    pub notional_value_dollars: String,
    pub yes_bid: u32,
    pub yes_bid_dollars: String,
    pub yes_ask: u32,
    pub yes_ask_dollars: String,
    pub no_bid: u32,
    pub no_bid_dollars: String,
    pub no_ask: u32,
    pub no_ask_dollars: String,
    pub last_price: u32,
    pub last_price_dollars: String,
    pub previous_yes_bid: u32,
    pub previous_yes_bid_dollars: String,
    pub previous_yes_ask: u32,
    pub previous_yes_ask_dollars: String,
    pub previous_price: u32,
    pub previous_price_dollars: String,
    pub volume: u64,
    pub volume_24h: u64,
    pub liquidity: u32,
    pub liquidity_dollars: String,
    pub open_interest: u32,
    pub can_close_early: bool,
    pub result: Option<String>,
    #[serde(default)]
    pub expiration_value: String,
    #[serde(default)]
    pub settlement_value: Option<u32>,
    #[serde(default)]
    pub settlement_value_dollars: Option<String>,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub risk_limit_cents: u32,
    #[serde(default)]
    pub strike_type: Option<String>,
    #[serde(default)]
    pub floor_strike: Option<f64>,
    #[serde(default)]
    pub cap_strike: Option<f64>,
    #[serde(default)]
    pub functional_strike: Option<String>,
    #[serde(default)]
    pub custom_strike: Option<HashMap<String, String>>,
    #[serde(default)]
    pub rules_primary: String,
    #[serde(default)]
    pub rules_secondary: String,
    pub tick_size: u32,
    pub price_level_structure: String,
    pub price_ranges: Vec<PriceRange>,
    #[serde(default)]
    pub fee_waiver_expiration_time: Option<String>,
    #[serde(default)]
    pub mve_collection_ticker: Option<String>,
    #[serde(default)]
    pub mve_selected_legs: Option<Vec<MveSelectedLeg>>,
    #[serde(default)]
    pub primary_participant_key: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MveSelectedLeg {
    pub event_ticker: String,
    pub market_ticker: String,
    pub side: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceRange {
    pub start: String,
    pub end: String,
    pub step: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("All markets {:?}", markets)]
pub struct GetMarketsResponse {
    pub cursor: Option<String>,
    pub markets: Vec<Market>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("Market {:?}", market)]
pub struct GetMarketResponse {
    pub market: Market,
}
#[derive(Serialize)]
pub struct MarketsQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_ticker: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_close_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_close_ts: Option<i64>,
    #[serde(rename = "statuses", skip_serializing_if = "Option::is_none")]
    pub status: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tickers: Option<String>,
}
#[derive(serde::Deserialize, Display, Debug)]
pub struct GetMarketOrderbookResponse {
    pub orderbook: Orderbook,
}
#[derive(serde::Deserialize, Display, Debug)]
#[display(
    "No(cents, total shares available) {:?} \nNo(dollars, shares available): {:?} \nYes(cents, total shares available) {:?} \nYes(dollars, shares available): {:?}",
    no,
    no_dollars,
    yes,
    yes_dollars
)]
pub struct Orderbook {
    pub no: Vec<(u64, u64)>,
    pub no_dollars: Vec<(String, u64)>,
    pub yes: Vec<(u64, u64)>,
    pub yes_dollars: Vec<(String, u64)>,
}
#[derive(Serialize)]
pub struct OrderbookQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u128>,
}
#[derive(Debug, Clone, Deserialize, Display)]
#[display(
    "candles: {} markets (adjusted_end_ts={})",
    "self.market_tickers.len()",
    "self.adjusted_end_ts"
)]
pub struct GetMarketCandlesticksResponse {
    pub market_candlesticks: Vec<Candlestick>,
    pub market_ticker: String,
}
#[derive(Debug, Clone, Deserialize, Display)]
#[display(
    "ts={} vol={} oi={:?} price[{}] bid[{}] ask[{}]",
    "self.end_period_ts",
    "self.volume",
    "self.open_interest",
    "self.price",
    "self.yes_bid",
    "self.yes_ask"
)]
pub struct Candlestick {
    pub end_period_ts: i64,
    pub open_interest: Option<u32>,
    pub volume: u32,
    pub price: PriceStats,
    pub yes_ask: SideOhlc,
    pub yes_bid: SideOhlc,
    #[serde(default)]
    pub no_ask: Option<SideOhlc>,
    #[serde(default)]
    pub no_bid: Option<SideOhlc>,
}
#[derive(Debug, Clone, Deserialize, Display)]
#[display(
    "prev={:?} o={:?} h={:?} l={:?} c={:?}",
    "self.previous",
    "self.open",
    "self.high",
    "self.low",
    "self.close"
)]
pub struct PriceStats {
    pub open: Option<u32>,
    pub open_dollars: Option<String>,
    pub close: Option<u32>,
    pub close_dollars: Option<String>,
    pub high: Option<u32>,
    pub high_dollars: Option<String>,
    pub low: Option<u32>,
    pub low_dollars: Option<String>,
    pub min: Option<u32>,
    pub min_dollars: Option<String>,
    pub max: Option<u32>,
    pub max_dollars: Option<String>,
    pub mean: Option<u32>,
    pub mean_dollars: Option<String>,
    pub previous: Option<u32>,
    pub previous_dollars: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Display)]
#[display("O/H/L/C={}/{}/{}/{}", "self.open", "self.high", "self.low", "self.close")]
pub struct SideOhlc {
    pub open: u32,
    pub open_dollars: String,
    pub high: u32,
    pub high_dollars: String,
    pub low: u32,
    pub low_dollars: String,
    pub close: u32,
    pub close_dollars: String,
}
#[derive(Serialize)]
pub struct CandlesticksQuery {
    pub start_ts: i64,
    pub end_ts: i64,
    pub period_interval: u32,
}
#[derive(Serialize)]
pub struct GetTradesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<u64>,
}
#[derive(Debug, Clone, Deserialize, Display)]
#[display("All trades available: {:?}", trades)]
pub struct GetTradesResponse {
    pub cursor: Option<String>,
    pub trades: Vec<Trade>,
}
#[derive(serde::Deserialize, Display, Debug, Clone)]
#[display("Trade: {} {} @ ${} ({})", ticker, count, price, take_side)]
pub struct Trade {
    pub count: i64,
    pub created_time: String,
    pub no_price: u64,
    pub no_price_dollars: String,
    pub price: f64,
    pub take_side: String,
    pub ticker: String,
    pub trade_id: String,
    pub yes_price: u64,
    pub yes_price_dollars: String,
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_market_deserialization_basic() {
        let json = r#"{
            "ticker": "TEST-TICKER",
            "event_ticker": "TEST-EVENT",
            "market_type": "binary",
            "title": "Test Market",
            "subtitle": "",
            "yes_sub_title": "Yes",
            "no_sub_title": "No",
            "open_time": "2025-01-01T00:00:00Z",
            "close_time": "2025-12-31T23:59:59Z",
            "expected_expiration_time": "2025-12-31T23:59:59Z",
            "expiration_time": "2025-12-31T23:59:59Z",
            "latest_expiration_time": "2025-12-31T23:59:59Z",
            "settlement_timer_seconds": 600,
            "status": "active",
            "response_price_units": "cents",
            "notional_value": 100,
            "notional_value_dollars": "1.00",
            "yes_bid": 50,
            "yes_bid_dollars": "0.50",
            "yes_ask": 55,
            "yes_ask_dollars": "0.55",
            "no_bid": 45,
            "no_bid_dollars": "0.45",
            "no_ask": 50,
            "no_ask_dollars": "0.50",
            "last_price": 52,
            "last_price_dollars": "0.52",
            "previous_yes_bid": 48,
            "previous_yes_bid_dollars": "0.48",
            "previous_yes_ask": 53,
            "previous_yes_ask_dollars": "0.53",
            "previous_price": 50,
            "previous_price_dollars": "0.50",
            "volume": 1000,
            "volume_24h": 500,
            "liquidity": 10000,
            "liquidity_dollars": "100.00",
            "open_interest": 250,
            "can_close_early": true,
            "expiration_value": "",
            "category": "sports",
            "risk_limit_cents": 5000,
            "strike_type": "binary",
            "tick_size": 1,
            "price_level_structure": "linear_cent",
            "price_ranges": [
                { "start": "0.0000", "end": "1.0000", "step": "0.0100" }
            ]
        }"#;
        let market: Market = serde_json::from_str(json).unwrap();
        assert_eq!(market.ticker, "TEST-TICKER");
        assert_eq!(market.event_ticker, "TEST-EVENT");
        assert_eq!(market.yes_bid, 50);
        assert_eq!(market.volume, 1000);
        assert_eq!(market.price_ranges.len(), 1);
    }
    #[test]
    fn test_market_with_mve_fields() {
        let json = r#"{
            "ticker": "MVE-TEST",
            "event_ticker": "MVE-EVENT",
            "market_type": "multi",
            "title": "Multi-variate",
            "subtitle": "",
            "yes_sub_title": "",
            "no_sub_title": "",
            "open_time": "2025-01-01T00:00:00Z",
            "close_time": "2025-12-31T23:59:59Z",
            "expected_expiration_time": "2025-12-31T23:59:59Z",
            "expiration_time": "2025-12-31T23:59:59Z",
            "latest_expiration_time": "2025-12-31T23:59:59Z",
            "settlement_timer_seconds": 600,
            "status": "active",
            "response_price_units": "cents",
            "notional_value": 100,
            "notional_value_dollars": "1.00",
            "yes_bid": 0,
            "yes_bid_dollars": "0.00",
            "yes_ask": 0,
            "yes_ask_dollars": "0.00",
            "no_bid": 0,
            "no_bid_dollars": "0.00",
            "no_ask": 0,
            "no_ask_dollars": "0.00",
            "last_price": 0,
            "last_price_dollars": "0.00",
            "previous_yes_bid": 0,
            "previous_yes_bid_dollars": "0.00",
            "previous_yes_ask": 0,
            "previous_yes_ask_dollars": "0.00",
            "previous_price": 0,
            "previous_price_dollars": "0.00",
            "volume": 0,
            "volume_24h": 0,
            "liquidity": 0,
            "liquidity_dollars": "0.00",
            "open_interest": 0,
            "can_close_early": false,
            "expiration_value": "",
            "category": "",
            "risk_limit_cents": 0,
            "strike_type": "custom",
            "tick_size": 1,
            "price_level_structure": "linear_cent",
            "price_ranges": [
                { "start": "0.0000", "end": "1.0000", "step": "0.0100" }
            ],
            "mve_collection_ticker": "MVE-COL",
            "mve_selected_legs": [
                {
                    "event_ticker": "E1",
                    "market_ticker": "M1",
                    "side": "yes"
                }
            ]
        }"#;
        let market: Market = serde_json::from_str(json).unwrap();
        assert_eq!(market.mve_collection_ticker, Some("MVE-COL".to_string()));
        assert_eq!(market.mve_selected_legs.as_ref().unwrap().len(), 1);
        assert_eq!(market.price_ranges.len(), 1);
    }
    #[test]
    fn test_markets_query_serialization() {
        let query = MarketsQuery {
            limit: Some(10),
            cursor: Some("abc123"),
            event_ticker: None,
            series_ticker: None,
            max_close_ts: None,
            min_close_ts: None,
            status: Some("active"),
            tickers: None,
        };
        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("limit=10"));
        assert!(encoded.contains("statuses=active"));
    }
    #[test]
    fn test_get_markets_response_deserialization() {
        let json = r#"{"markets": []}"#;
        let response: GetMarketsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.markets.len(), 0);
    }
}
