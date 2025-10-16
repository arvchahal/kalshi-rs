use crate::auth::Account;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct OrderRequest {}

pub struct Position {
    user: Account
}

pub struct OrderStatus {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub ticker: String,
    pub event_ticker: String,
    pub market_type: String,
    pub title: String,
    pub subtitle: String,
    pub yes_sub_title: String,
    pub no_sub_title: String,

    // Times - using String since Kalshi returns ISO 8601 format
    pub open_time: String,
    pub close_time: String,
    pub expected_expiration_time: String,
    pub expiration_time: String,
    pub latest_expiration_time: String,

    pub settlement_timer_seconds: u32,
    pub status: String,
    pub response_price_units: String,

    // Notional values
    pub notional_value: u32,
    pub notional_value_dollars: String,

    // Prices (in cents)
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

    // Previous prices
    pub previous_yes_bid: u32,
    pub previous_yes_bid_dollars: String,
    pub previous_yes_ask: u32,
    pub previous_yes_ask_dollars: String,
    pub previous_price: u32,
    pub previous_price_dollars: String,

    // Volume and liquidity
    pub volume: u64,
    pub volume_24h: u64,
    pub liquidity: u32,
    pub liquidity_dollars: String,
    pub open_interest: u32,

    pub result: String,
    pub can_close_early: bool,
    pub expiration_value: String,
    pub category: String,
    pub risk_limit_cents: u32,
    pub strike_type: String,

    // Optional fields that may not always be present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_strike: Option<HashMap<String, String>>,

    #[serde(default)]
    pub rules_primary: String,
    #[serde(default)]
    pub rules_secondary: String,

    pub tick_size: u32,

    // MVE (Multi-Variate Event) fields - optional since not all markets have these
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mve_collection_ticker: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mve_selected_legs: Option<Vec<MveSelectedLeg>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MveSelectedLeg {
    pub event_ticker: String,
    pub market_ticker: String,
    pub side: String, // "yes" or "no"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMarketResponse {
    pub markets: Vec<Market>,
}

