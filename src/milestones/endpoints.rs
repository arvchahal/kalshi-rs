//! Milestones module endpoints.
//!
//! This module implements API endpoints for milestones operations.

use crate::client::KalshiClient;
use crate::errors::KalshiError;
use crate::milestones::models::{
    GetMilestoneResponse, GetMilestonesResponse, MilestonesQuery,
};

const GET_MILESTONE: &str = "/trade-api/v2/milestones/{}";
const GET_MILESTONES: &str = "/trade-api/v2/milestones";


impl KalshiClient {
    
    /// Get Milestone.
    ///
    /// **Endpoint:** `GET /milestones/{}`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_milestone(
        &self,
        id: &str,
    ) -> Result<GetMilestoneResponse, KalshiError> {
        let url = GET_MILESTONE.replace("{}", id);
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetMilestoneResponse = serde_json::from_str(&resp)
            .map_err(|e| {
                KalshiError::Other(
                    format!(
                        "Invalid Parsing response format: Parse error: {e}. Response: {resp}"
                    ),
                )
            })?;
        Ok(data)
    }


    /// Get Milestones.
    ///
    /// **Endpoint:** `GET /milestones/{}`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_milestones(
        &self,
        limit: Option<u32>,
    ) -> Result<GetMilestonesResponse, KalshiError> {
        let params = MilestonesQuery { limit };
        let query = serde_urlencoded::to_string(&params)
            .map_err(|e| KalshiError::Other(
                format!("Failed to serialize params: {}", e),
            ))?;
        let url = if query.is_empty() {
            GET_MILESTONES.to_string()
        } else {
            format!("{}?{}", GET_MILESTONES, query)
        };
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetMilestonesResponse = serde_json::from_str(&resp)
            .map_err(|e| {
                KalshiError::Other(
                    format!(
                        "Invalid Parsing response format: Parse error: {e}. Response: {resp}"
                    ),
                )
            })?;
        Ok(data)
    }
}
