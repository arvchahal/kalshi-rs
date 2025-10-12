use crate::client::KalshiClient;
use crate::api_keys::models::{
    ApiKey, CreateApiKeyRequest, CreateApiKeyResponse, ListApiKeysResponse
};
use crate::errors::KalshiError;

impl KalshiClient {
    /// GET /users/api-keys
    /// List all API keys for the authenticated user
    /// Requires: KALSHI-ACCESS-KEY, KALSHI-ACCESS-TIMESTAMP, KALSHI-ACCESS-SIGNATURE
    pub async fn list_api_keys(&self) -> Result<Vec<ApiKey>, KalshiError> {
        let path = "/users/api-keys";
        let url = format!("{}{}", self.base_url, path);

        // TODO: Add authentication headers:
        // - KALSHI-ACCESS-KEY: the Key ID
        // - KALSHI-ACCESS-TIMESTAMP: request timestamp in ms
        // - KALSHI-ACCESS-SIGNATURE: request hash signed with private key
        let response = self.http_client
            .get(&url)
            .send()
            .await?;

        let data: ListApiKeysResponse = response.json().await?;
        Ok(data.keys)
    }

    /// POST /users/api-keys
    /// Create a new API key
    /// Requires: KALSHI-ACCESS-KEY, KALSHI-ACCESS-TIMESTAMP, KALSHI-ACCESS-SIGNATURE
    pub async fn create_api_key(&self, description: Option<String>) -> Result<CreateApiKeyResponse, KalshiError> {
        let path = "/users/api-keys";
        let url = format!("{}{}", self.base_url, path);

        let request = CreateApiKeyRequest { description };

        // TODO: Add authentication headers
        let response = self.http_client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    /// DELETE /users/api-keys/{key_id}
    /// Revoke an API key
    /// Requires: KALSHI-ACCESS-KEY, KALSHI-ACCESS-TIMESTAMP, KALSHI-ACCESS-SIGNATURE
    pub async fn revoke_api_key(&self, key_id: &str) -> Result<(), KalshiError> {
        let path = format!("/users/api-keys/{}", key_id);
        let url = format!("{}{}", self.base_url, path);

        // TODO: Add authentication headers
        let response = self.http_client
            .delete(&url)
            .send()
            .await?;

        response.error_for_status()?;
        Ok(())
    }

    /// GET /users/api-keys/{key_id}
    /// Get details for a specific API key
    /// Requires: KALSHI-ACCESS-KEY, KALSHI-ACCESS-TIMESTAMP, KALSHI-ACCESS-SIGNATURE
    pub async fn get_api_key(&self, key_id: &str) -> Result<ApiKey, KalshiError> {
        let path = format!("/users/api-keys/{}", key_id);
        let url = format!("{}{}", self.base_url, path);

        // TODO: Add authentication headers
        let response = self.http_client
            .get(&url)
            .send()
            .await?;

        Ok(response.json().await?)
    }
}
