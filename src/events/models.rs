use serde::{Serialize, Deserialize};
use derive_more::Display;
use crate::markets::models::Market;

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("Metadata: competition={} scope={} sources={}", 
          competition.as_deref().unwrap_or("none"), 
          competition_scope.as_deref().unwrap_or("none"), 
          settlement_sources.len())]
pub struct GetEventMetadataResponse {
    pub image_url: Option<String>,
    pub settlement_sources: Vec<SettlementSource>,
    pub competition: Option<String>,
    pub competition_scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{} ({})", name, url)]
pub struct SettlementSource {
    pub name: String,
    pub url: String,
}



#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("Event {} with {} markets", event.event_ticker, markets.len())]
pub struct GetEventResponse {
    pub event: Event,
    pub markets: Vec<Market>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{} ({})", title, event_ticker)]
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
    pub markets: Vec<Market>,
    pub available_on_brokers: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("All events: {}", events.len())]
pub struct GetEventsResponse {
    pub events: Vec<Event>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MveSelectedLeg {
    pub event_ticker: String,
    pub market_ticker: String,
    pub side: String, // "yes" or "no"
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceRange {
    pub start: String,
    pub end: String,
    pub step: String,
}