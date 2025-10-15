use crate::client::KalshiClient;
use crate::api_keys::models::{ApiKey, CreateApiKeyRequest, CreateApiKeyResponse, ListApiKeysResponse};
use crate::errors::KalshiError;

// Endpoints for creating/deleting/generating/getting api keys
// All of these methods require auth including your private key,
// your signed text with your key, and the timestamp

/*
From Kalshi docs:
KALSHI-ACCESS-KEY - the Key ID
KALSHI-ACCESS-TIMESTAMP - the request timestamp in ms
KALSHI-ACCESS-SIGNATURE - request hash signed with private key
*/

const GET_API_KEY: &str = "/trade-api/v2/api_keys/"; // get
const CREATE_API_KEY: &str = "/trade-api/v2/api_keys/"; // post
const DELETE_API_KEY: &str = "/trade-api/v2/api_keys/generate/"; // post
const GENERATE_API_KEY: &str = "/trade-api/v2/api_keys/{api_key}/"; // delete

fn full_url(prefix: &str, suffix: &str) -> String {
   let x = format!("{}{}", prefix, suffix); // e.g. "/trade-api/v2/api_keys"
    println!("{}",x);
    x
}




impl KalshiClient {
    /// GET /api-keys
    /// List all API keys for the authenticated user
    pub async fn get_api_keys(&self) -> Result<Vec<ApiKey>, KalshiError> {
        let body = self.authenticated_get(GET_API_KEY).await?;

        let data: ListApiKeysResponse = serde_json::from_str(&body)
            .map_err(|e| KalshiError::Other(format!("Parse error: {e}. Body: {body}")))?;
        Ok(data.api_keys)
    }

    /// POST /users/api-keys/generate
    /// Generate a new API key (alternative endpoint)
    pub async fn generate_api_key(&self) -> Result<CreateApiKeyResponse, KalshiError> {
        let body = self.authenticated_post(DELETE_API_KEY, None::<&()>).await?;

        let data: CreateApiKeyResponse = serde_json::from_str(&body)
            .map_err(|e| KalshiError::Other(format!("Parse error: {e}. Body: {body}")))?;
        Ok(data)
    }
}
