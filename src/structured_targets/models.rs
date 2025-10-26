use derive_more::Display;
use serde::Deserialize;

#[derive(Deserialize, Display, Debug, Clone)]
#[display("StructuredTarget {{ id: {id}, name: {name}, type: {type}, source_id: {source_id}, last_updated_ts: {last_updated_ts} }}")]
pub struct StructuredTarget {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub details: serde_json::Value,      // arbitrary JSON object
    pub source_id: String,
    pub last_updated_ts: String,  // enable chrono's "serde" feature
}

#[derive(Deserialize, Display, Debug, Clone)]
#[display("GetStructuredTargetsResponse {{ cursor: {cursor:?}, structured_targets: {structured_targets:?} }}")]
pub struct GetStructuredTargetsResponse {
    pub structured_targets: Vec<StructuredTarget>,
    pub cursor: Option<String>,
}

#[derive(Deserialize, Display, Debug, Clone)]
#[display("GetStructuredTargetResponse {{ structured_target: {structured_target} }}")]
pub struct GetStructuredTargetResponse {
    pub structured_target: StructuredTarget,
}