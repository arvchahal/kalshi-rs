//! Events module models.
//!
//! This module contains data structures for events functionality.

use crate::markets::models::Market;
use derive_more::Display;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display(
    "Metadata: competition={} scope={} sources={}",
    competition.as_deref().unwrap_or("none"),
    competition_scope.as_deref().unwrap_or("none"),
    settlement_sources.len()
)]


/// Response model for API endpoint.
///
pub struct GetEventMetadataResponse {
    pub image_url: Option<String>,
    pub settlement_sources: Vec<SettlementSource>,
    pub competition: Option<String>,
    pub competition_scope: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{} ({})", name, url)]


/// SettlementSource data model.
///
pub struct SettlementSource {
    pub name: String,
    pub url: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("Event {} with {} markets", event.event_ticker, markets.len())]


/// Response model for API endpoint.
///
pub struct GetEventResponse {
    pub event: Event,
    pub markets: Vec<Market>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{} ({})", title, event_ticker)]


/// Event data model.
///
pub struct Event {
    pub event_ticker: String,
    pub series_ticker: String,
    pub sub_title: Option<String>,
    pub title: String,
    pub collateral_return_type: Option<String>,
    pub mutually_exclusive: bool,
    pub category: Option<String>,
    pub strike_date: Option<String>,
    pub strike_period: Option<String>,
    #[serde(default)]
    pub markets: Vec<Market>,
    pub available_on_brokers: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("All events: {}", events.len())]


/// Response model for API endpoint.
///
pub struct GetEventsResponse {
    pub events: Vec<Event>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]


pub struct MveSelectedLeg {
    pub event_ticker: String,
    pub market_ticker: String,
    pub side: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]


/// PriceRange data model.
///
pub struct PriceRange {
    pub start: String,
    pub end: String,
    pub step: String,
}


#[derive(Debug, Serialize)]


/// Query parameters for API endpoint.
///
pub struct EventsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}
