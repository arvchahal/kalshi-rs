use derive_more::Display;
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

#[derive(Debug, Deserialize, Display)]
#[display("CreateApiKeyResponse {{ key_id: {}, api_key: {}}}", api_key_id, private_key)]
pub struct CreateApiKeyResponse {
    pub api_key_id: String,
    pub private_key: String,
}

#[derive(Debug, Deserialize)]
pub struct ListApiKeysResponse {
    pub api_keys: Vec<ApiKey>,
}

#[derive(Debug, Deserialize, Display)]
#[display("Delete API key response {{{:?}, delete sucessful}}", &self.body)]
pub struct DeleteApiKeyResponse{
    pub body: Option<String>,
}