//! Communications module models.
//!
//! This module contains data structures for communications functionality.

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCommunicationsIDResponse {
    #[serde(alias = "communcation_id")]
    pub communications_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRFQResponse {
    pub rfq: RFQ,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQuoteResponse {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQuoteRequest {
    pub rfq_id: String,
    pub yes_bid: String,
    pub no_bid: String,
    pub rest_remainder: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteQuoteResponse {
    pub body: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]

/// Response model for API endpoint.
///
pub struct DeleteRFQResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptQuoteResponse {
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Accept {
    Yes,
    No,
}

impl Accept {
    pub fn from_str_helper(s: &str) -> Result<Accept, String> {
        match s {
            "yes" | "Yes" | "YES" => Ok(Accept::Yes),
            "no" | "No" | "NO" => Ok(Accept::No),
            _ => Err(format!("Invalid value: {}, choose from Yes or No", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmQuoteResponse {
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRFQRequest {
    pub market_ticker: String,
    pub rest_remainder: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_cost_centi_cents: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_existing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtrader_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRFQResponse {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRFQsResponse {
    pub rfqs: Vec<RFQ>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RFQ {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_user_id: Option<String>,
    pub creator_id: String,
    pub market_ticker: String,
    pub contacts: Option<u64>,
    pub rest_remainder: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_cost_centi_cents: Option<i32>,
    pub mve_collection_ticker: Option<String>,
    pub mve_selected_legs: Option<Vec<MveLogs>>,
    pub cancellation_reason: Option<String>,
    #[serde(alias = "cancelled_time")]
    pub cancelled_ts: Option<String>,
    #[serde(alias = "updated_time")]
    pub updated_ts: Option<String>,
    #[serde(alias = "created_time")]
    pub created_ts: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MveLogs {
    event_ticker: Option<String>,
    market_ticker: Option<String>,
    side: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetQuotesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_creator_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfq_creator_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetQuotesResponse {
    pub quotes: Vec<Quote>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetQuoteResponse {
    pub quote: Quote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub id: String,
    pub rfq_id: String,
    pub creator_id: String,
    pub rfq_creator_id: String,
    pub market_ticker: String,
    pub contracts: i32,
    pub yes_bid: i32,
    pub no_bid: i32,
    pub created_ts: String,
    pub updated_ts: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_ts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmed_ts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executed_ts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_ts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_remainder: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfq_creator_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_ts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfq_target_cost_centi_cents: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfq_creator_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_order_id: Option<String>,
}
