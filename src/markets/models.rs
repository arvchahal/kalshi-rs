//! Markets module models.
//!
//! This module contains data structures for Kalshi market data, including market information,
//! pricing, orderbooks, candlesticks, and trades.

use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


/// Represents a single market on the Kalshi platform.
///
/// Contains comprehensive market data including pricing, volume, liquidity,
/// and settlement information for binary prediction markets.
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
    pub liquidity: i64,
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


/// Represents a selected leg in a multivariate event (MVE) collection.
///
/// Used for markets that are part of composite multi-outcome events.
#[derive(Debug, Clone, Serialize, Deserialize)]


pub struct MveSelectedLeg {
    pub event_ticker: String,
    pub market_ticker: String,
    pub side: String,
}


/// Defines a price range for market trading with specified step increments.
#[derive(Debug, Clone, Serialize, Deserialize)]


pub struct PriceRange {
    pub start: String,
    pub end: String,
    pub step: String,
}


/// Response from `GET /markets` endpoint.
///
/// Contains a list of markets with optional cursor for pagination.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("All markets {:?}", markets)]


pub struct GetMarketsResponse {
    pub cursor: Option<String>,
    pub markets: Vec<Market>,
}


/// Response from `GET /markets/{ticker}` endpoint.
///
/// Returns detailed information for a single market.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("Market {:?}", market)]


pub struct GetMarketResponse {
    pub market: Market,
}


/// Query parameters for `GET /markets` endpoint.
///
/// All fields are optional filters for market retrieval.
#[derive(Serialize, Default)]


pub struct MarketsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_close_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_close_ts: Option<i64>,
    #[serde(rename = "statuses", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tickers: Option<String>,
}


/// Response from `GET /markets/{ticker}/orderbook` endpoint.
///
/// Contains the current orderbook with bid/ask levels.
#[derive(serde::Deserialize, Display, Debug)]


pub struct GetMarketOrderbookResponse {
    pub orderbook: Orderbook,
}


/// Market orderbook containing yes/no bid and ask levels.
///
/// Each level is represented as a tuple of (price, quantity).
/// Prices are available in both cents and dollar representations.
#[derive(serde::Deserialize, Display, Debug)]
#[display(
    "No(cents, total shares available) {:?} \nNo(dollars, shares available): {:?} \nYes(cents, total shares available) {:?} \nYes(dollars, shares available): {:?}",
    no,
    no_dollars,
    yes,
    yes_dollars
)]


/// Orderbook data model.
///
pub struct Orderbook {
    pub no: Option<Vec<(u64, u64)>>,
    pub no_dollars: Option<Vec<(String, u64)>>,
    pub yes: Option<Vec<(u64, u64)>>,
    pub yes_dollars: Option<Vec<(String, u64)>>,
}


/// Query parameters for `GET /markets/{ticker}/orderbook` endpoint.
#[derive(Serialize)]


pub struct OrderbookQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u128>,
}


/// Response from `GET /series/{series_ticker}/markets/{ticker}/candlesticks` endpoint.
///
/// Returns historical candlestick (OHLC) data for a market.
#[derive(Debug, Clone, Deserialize, Display)]
#[display(
    "candles: {} markets (adjusted_end_ts={})",
    "self.market_tickers.len()",
    "self.adjusted_end_ts"
)]


/// Response model for API endpoint.
///
pub struct GetMarketCandlesticksResponse {
    pub market_candlesticks: Vec<Candlestick>,
    pub market_ticker: String,
}


/// A single candlestick data point representing market activity for a time period.
///
/// Contains OHLC (Open, High, Low, Close) data for both yes and no sides,
/// along with volume and open interest statistics.
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


/// Candlestick data model.
///
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


/// Statistical price information for a candlestick period.
///
/// Contains OHLC values, min/max/mean statistics, and previous period comparisons.
#[derive(Debug, Clone, Deserialize, Display)]
#[display(
    "prev={:?} o={:?} h={:?} l={:?} c={:?}",
    "self.previous",
    "self.open",
    "self.high",
    "self.low",
    "self.close"
)]


/// PriceStats data model.
///
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


/// OHLC (Open, High, Low, Close) data for one side of the market (yes or no).
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


/// Query parameters for `GET /series/{series_ticker}/markets/{ticker}/candlesticks` endpoint.
#[derive(Serialize)]
pub struct CandlesticksQuery {
    pub start_ts: i64,
    pub end_ts: i64,
    pub period_interval: u32,
}


/// Query parameters for `GET /markets/trades` endpoint.
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


/// Response from `GET /markets/trades` endpoint.
///
/// Returns a list of recent trades with optional pagination cursor.
#[derive(Debug, Clone, Deserialize, Display)]
#[display("All trades available: {:?}", trades)]
pub struct GetTradesResponse {
    pub cursor: Option<String>,
    pub trades: Vec<Trade>,
}


/// Represents a single trade execution on a market.
///
/// Contains trade details including price, quantity, side, and timestamp.
#[derive(serde::Deserialize, Display, Debug, Clone)]
#[display("Trade: {} {} @ ${} ({})", ticker, count, price, taker_side)]
pub struct Trade {
    pub count: i64,
    pub created_time: String,
    pub no_price: u64,
    pub no_price_dollars: String,
    pub price: f64,
    pub taker_side: String,
    pub ticker: String,
    pub trade_id: String,
    pub yes_price: u64,
    pub yes_price_dollars: String,
}
