//! Api_keys module models.
//!
//! This module contains data structures for api_keys functionality.

use derive_more::Display;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
/// ApiKey data model.
///
pub struct ApiKey {
    pub api_key_id: String,
    pub name: String,
}


#[derive(Debug, Serialize)]
/// Request model for create API key endpoint.
///
pub struct CreateApiKeyRequest {
    pub description: Option<String>,
}


#[derive(Debug, Deserialize, Display)]
#[display("CreateApiKeyResponse {{ key_id: {}, api_key: {}}}", api_key_id, private_key)]


/// Response model for API endpoint.
///
pub struct CreateApiKeyResponse {
    pub api_key_id: String,
    pub private_key: String,
}


#[derive(Debug, Deserialize)]
/// Response model for list API keys.
///
pub struct ListApiKeysResponse {
    pub api_keys: Vec<ApiKey>,
}


#[derive(Debug, Deserialize, Display)]
#[display("Delete API key response {{{:?}, delete sucessful}}", &self.body)]
/// Response model for delete API key endpoint.
///
pub struct DeleteApiKeyResponse {
    pub body: Option<String>,
}
