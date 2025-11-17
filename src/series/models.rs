//! Series module models.
//!
//! This module contains data structures for series functionality.

use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Debug, Clone, Serialize, Deserialize)]


/// SettlementSource data model.
///
pub struct SettlementSource {
    pub name: Option<String>,
    pub url: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]


pub struct Series {
    pub ticker: String,
    pub frequency: String,
    pub title: String,
    pub category: String,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub settlement_sources: Option<Vec<SettlementSource>>,
    pub contract_url: Option<String>,
    pub contract_terms_url: Option<String>,
    pub fee_type: String,
    pub fee_multiplier: f32,
    #[serde(default)]
    pub additional_prohibitions: Option<Vec<String>>,
    #[serde(default)]
    pub product_metadata: Option<serde_json::Value>,
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


#[derive(Debug, Serialize)]
pub struct SeriesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}
