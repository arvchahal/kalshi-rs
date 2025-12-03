//! Structured_targets module models.
//!
//! This module contains data structures for structured_targets functionality.

use derive_more::Display;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Display, Debug, Clone)]
#[display(
    "StructuredTarget {{ id: {id}, name: {name}, type: {type}, source_id: {source_id:?}, last_updated_ts: {last_updated_ts} }}"
)]
/// StructuredTarget data model.
///
pub struct StructuredTarget {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub details: serde_json::Value,
    pub source_id: Option<String>,
    pub last_updated_ts: String,
}


#[derive(Deserialize, Display, Debug, Clone)]
#[display(
    "GetStructuredTargetsResponse {{ cursor: {cursor:?}, structured_targets: {structured_targets:?} }}"
)]
/// Response model for API endpoint.
///
pub struct GetStructuredTargetsResponse {
    pub structured_targets: Vec<StructuredTarget>,
    pub cursor: Option<String>,
}


#[derive(Deserialize, Display, Debug, Clone)]
#[display("GetStructuredTargetResponse {{ structured_target: {structured_target} }}")]
pub struct GetStructuredTargetResponse {
    pub structured_target: StructuredTarget,
}


#[derive(Debug, Serialize)]
pub struct StructuredTargetsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}
