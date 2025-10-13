use std::error::Error;
use crate::client::KalshiClient;
use crate::api_keys::models::{ApiKey, CreateApiKeyRequest, CreateApiKeyResponse, ListApiKeysResponse};
use crate::errors::KalshiError;
use crate::auth::auth_loader::{get_current_timestamp_ms,sign_request};

// Endpoints for creating/deleting/generating/getting api keys
// All of these methods require auth including your private key,
// your signed text with your key, and the timestamp

/*
From Kalshi docs:
KALSHI-ACCESS-KEY - the Key ID
KALSHI-ACCESS-TIMESTAMP - the request timestamp in ms
KALSHI-ACCESS-SIGNATURE - request hash signed with private key
*/

const GET_API_KEY: &str = "api_keys"; // get
const CREATE_API_KEY: &str = "api_keys"; // post
const DELETE_API_KEY: &str = "api_keys/generate"; // post
const GENERATE_API_KEY: &str = "api_keys/{api_key}"; // delete





impl KalshiClient {
    /// GET /api-keys
    /// List all API keys for the authenticated user
    pub async fn get_api_keys(&self) -> Result<Vec<ApiKey>, KalshiError> {
        let url = format!("{}/{}", self.base_url, GET_API_KEY);

        let timestamp_str = get_current_timestamp_ms();
        let timestamp: u64 = timestamp_str.parse()
            .map_err(|e| KalshiError::Other(format!("Failed to parse timestamp: {}", e)))?;

        let key_id = self.account.key_id();
        let signature = sign_request(&self.account.private_key_pem(), "GET", GET_API_KEY, timestamp)
            .map_err(|e| KalshiError::Other(format!("Failed to sign request: {}", e)))?;

        let response = self.http_client
            .get(&url)
            .header("KALSHI-ACCESS-KEY", key_id)
            .header("KALSHI-ACCESS-TIMESTAMP", timestamp_str)
            .header("KALSHI-ACCESS-SIGNATURE", signature)
            .send()
            .await?;

        let data: ListApiKeysResponse = response.json().await?;
        Ok(data.keys)
    }

    /// POST /api-keys
    /// Create a new API key
    pub async fn create_api_key(&self, description: Option<String>) -> Result<CreateApiKeyResponse, KalshiError> {
        let url = format!("{}/{}", self.base_url, CREATE_API_KEY);
        let request_body = CreateApiKeyRequest { description };

        let timestamp_str = get_current_timestamp_ms();
        let timestamp: u64 = timestamp_str.parse()
            .map_err(|e| KalshiError::Other(format!("Failed to parse timestamp: {}", e)))?;

        let key_id = self.account.key_id();
        let signature = sign_request(&self.account.private_key_pem(), "POST", CREATE_API_KEY, timestamp)
            .map_err(|e| KalshiError::Other(format!("Failed to sign request: {}", e)))?;

        let response = self.http_client
            .post(&url)
            .header("KALSHI-ACCESS-KEY", key_id)
            .header("KALSHI-ACCESS-TIMESTAMP", timestamp_str)
            .header("KALSHI-ACCESS-SIGNATURE", signature)
            .json(&request_body)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    /// DELETE /users/api-keys/{key_id}
    /// Delete/revoke an API key
    pub async fn delete_api_key(&self, key_id: &str) -> Result<(), KalshiError> {
        let path = GENERATE_API_KEY.replace("{api_key}", key_id);
        let url = format!("{}/{}", self.base_url, path);

        let timestamp_str = get_current_timestamp_ms();
        let timestamp: u64 = timestamp_str.parse()
            .map_err(|e| KalshiError::Other(format!("Failed to parse timestamp: {}", e)))?;

        let account_key_id = self.account.key_id();
        let signature = sign_request(&self.account.private_key_pem(), "DELETE", &path, timestamp)
            .map_err(|e| KalshiError::Other(format!("Failed to sign request: {}", e)))?;

        let response = self.http_client
            .delete(&url)
            .header("KALSHI-ACCESS-KEY", account_key_id)
            .header("KALSHI-ACCESS-TIMESTAMP", timestamp_str)
            .header("KALSHI-ACCESS-SIGNATURE", signature)
            .send()
            .await?;

        response.error_for_status()?;
        Ok(())
    }

    /// POST /users/api-keys/generate
    /// Generate a new API key (alternative endpoint)
    pub async fn generate_api_key(&self) -> Result<CreateApiKeyResponse, KalshiError> {
        let url = format!("{}/{}", self.base_url, DELETE_API_KEY);

        let timestamp_str = get_current_timestamp_ms();
        let timestamp: u64 = timestamp_str.parse()
            .map_err(|e| KalshiError::Other(format!("Failed to parse timestamp: {}", e)))?;

        let key_id = self.account.key_id();
        let signature = sign_request(&self.account.private_key_pem(), "POST", DELETE_API_KEY, timestamp)
            .map_err(|e| KalshiError::Other(format!("Failed to sign request: {}", e)))?;

        let response = self.http_client
            .post(&url)
            .header("KALSHI-ACCESS-KEY", key_id)
            .header("KALSHI-ACCESS-TIMESTAMP", timestamp_str)
            .header("KALSHI-ACCESS-SIGNATURE", signature)
            .send()
            .await?;

        Ok(response.json().await?)
    }
}
