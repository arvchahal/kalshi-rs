use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub api_key_id: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateApiKeyRequest {
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateApiKeyResponse {
    pub key_id: String,
    pub api_key: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ListApiKeysResponse {
    pub api_keys: Vec<ApiKey>,
}
