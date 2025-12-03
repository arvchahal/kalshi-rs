//! Api_keys module endpoints.
//!
//! This module implements API endpoints for api_keys operations.

use crate::api_keys::models::{
    ApiKey, CreateApiKeyResponse, DeleteApiKeyResponse, ListApiKeysResponse,
};


use crate::client::KalshiClient;
use crate::errors::KalshiError;
use serde_json::json;


const GET_API_KEY: &str = "/trade-api/v2/api_keys";
const GENERATE_API_KEY: &str = "/trade-api/v2/api_keys/generate";
const DELETE_API_KEY: &str = "/trade-api/v2/api_keys/{}";

impl KalshiClient {

    /// GET /api-keys
    /// List all API keys for the authenticated user
    pub async fn get_api_keys(&self) -> Result<Vec<ApiKey>, KalshiError> {
        let body: String = self.authenticated_get::<str>(GET_API_KEY, None).await?;
        let data: ListApiKeysResponse = serde_json::from_str(&body)
            .map_err(|e| KalshiError::Other(format!("Parse error: {e}. Body: {body}")))?;
        Ok(data.api_keys)
    }


    /// POST /users/api-keys/generate
    /// Generate a new API key (alternative endpoint)
    pub async fn generate_api_key(
        &self,
        api_key_name: &str,
    ) -> Result<CreateApiKeyResponse, KalshiError> {
        let body = json!({ "name" : api_key_name });
        let resp: String = self.authenticated_post(GENERATE_API_KEY, Some(&body)).await?;
        let data: CreateApiKeyResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Parse error: {e}. Response: {resp}"),
            ))?;
        Ok(data)
    }


    /// DELETE /trade-api/v2/api_keys/{api_key}/
    /// delete an API key if it exists else returns error
    ///
    pub async fn delete_api_key(
        &self,
        api_key: &str,
    ) -> Result<DeleteApiKeyResponse, KalshiError> {
        let url = DELETE_API_KEY.replace("{}", api_key);
        let (status, resp) = self
            .authenticated_delete(&url, Option::<&str>::None)
            .await?;
        if status.as_u16() == 204 || resp.trim().is_empty() {
            return Ok(DeleteApiKeyResponse { body: None });
        }
        let data: DeleteApiKeyResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Parse error: {e}. Response: {resp}"),
            ))?;
        Ok(data)
    }
}
