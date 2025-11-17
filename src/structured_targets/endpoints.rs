//! Structured_targets module endpoints.
//!
//! This module implements API endpoints for structured_targets operations.

use crate::KalshiClient;
use crate::errors::KalshiError;
use crate::structured_targets::models::{
    GetStructuredTargetResponse, GetStructuredTargetsResponse, StructuredTargetsQuery,
};


const STRUCTURED_TARGETS: &str = "/trade-api/v2/structured_targets";


const STRUCTURED_TARGET: &str = "/trade-api/v2/structured_targets/{}";


impl KalshiClient {

    
    /// Get All Structured Targets.
    ///
    /// **Endpoint:** `GET /structured_targets`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_all_structured_targets(
        &self,
        limit: Option<u64>,
        cursor: Option<&str>,
    ) -> Result<GetStructuredTargetsResponse, KalshiError> {
        let params = StructuredTargetsQuery {
            limit,
            cursor: cursor.map(|s| s.to_string()),
        };
        let query = serde_urlencoded::to_string(&params)
            .map_err(|e| KalshiError::Other(
                format!("Failed to serialize params: {}", e),
            ))?;
        let url = if query.is_empty() {
            STRUCTURED_TARGETS.to_string()
        } else {
            format!("{}?{}", STRUCTURED_TARGETS, query)
        };
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetStructuredTargetsResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Parse error: {e}. Response: {resp}"),
            ))?;
        Ok(data)
    }


    /// Get Structured Target.
    ///
    /// **Endpoint:** `GET /structured_targets/{}`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_structured_target(
        &self,
        structured_target_id: &str,
    ) -> Result<GetStructuredTargetResponse, KalshiError> {
        let url: String = STRUCTURED_TARGET.replace("{}", structured_target_id);
        let resp: String = self.unauthenticated_get(&url).await?;
        let data: GetStructuredTargetResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Parse error: {e}. Response: {resp}"),
            ))?;
        Ok(data)
    }
}
