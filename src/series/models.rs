use serde::{Serialize, Deserialize};
use derive_more::Display;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementSource {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Series {
    pub ticker: String,
    pub frequency: String,
    pub title: String,
    pub category: String,
    pub tags: Vec<String>,
    pub settlement_sources: Vec<SettlementSource>,
    pub contract_url: String,
    pub contract_terms_url: String,
    pub product_metadata: HashMap<String, String>,
    pub fee_type: String,
    pub fee_multiplier: u32,
    pub additional_prohibitions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("All series retrieved ({}) entries", series.len())]
pub struct GetSeriesListResponse {
    pub series: Vec<Series>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("Series details for {}", series.ticker)]
pub struct GetSeriesResponse {
    pub series: Series,
}