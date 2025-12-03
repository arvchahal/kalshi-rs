//! Exchange module endpoints.
//!
//! This module implements API endpoints for exchange operations.

use std::vec;
use crate::client::KalshiClient;
use crate::errors::KalshiError;
use crate::exchange::models::{
    GetExcahngeStatus, GetExchangeAnnouncementsResponse, GetExchangeScheduleResponse,
    GetUserDataTimestampResponse,
};


const GET_EXCHANGE_ANNOUNCEMENTS: &str = "/trade-api/v2/exchange/announcements";
const GET_EXCHANGE_SCHEDULE: &str = "/trade-api/v2/exchange/schedule";
const GET_EXCHANGE_STATUS: &str = "/trade-api/v2/exchange/status";
const GET_USER_DATA_TIMESTAMP: &str = "/trade-api/v2/exchange/user_data_timestamp";

impl KalshiClient {

    /// Get Exchange Announcements.
    ///
    /// **Endpoint:** `GET /exchange/announcements`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_exchange_announcements(
        &self,
    ) -> Result<GetExchangeAnnouncementsResponse, KalshiError> {
        let resp = self.unauthenticated_get(GET_EXCHANGE_ANNOUNCEMENTS).await?;
        if resp.trim().is_empty() {
            return Ok(GetExchangeAnnouncementsResponse {
                announcements: vec![],
            });
        }
        let data: GetExchangeAnnouncementsResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Parse error: {e}. Response: {resp}"),
            ))?;
        Ok(data)
    }


    /// Get Exchange Schedule.
    ///
    /// **Endpoint:** `GET /exchange/schedule`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_exchange_schedule(
        &self,
    ) -> Result<GetExchangeScheduleResponse, KalshiError> {
        let resp = self.unauthenticated_get(GET_EXCHANGE_SCHEDULE).await?;
        let data: GetExchangeScheduleResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Parse error: {e}. Response: {resp}"),
            ))?;
        Ok(data)
    }


    /// Get Exchange Status.
    ///
    /// **Endpoint:** `GET /exchange/status`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_exchange_status(&self) -> Result<GetExcahngeStatus, KalshiError> {
        let resp = self.unauthenticated_get(GET_EXCHANGE_STATUS).await?;
        let data: GetExcahngeStatus = serde_json::from_str(&resp)
            .map_err(|e| {
                KalshiError::Other(
                    format!(
                        "Invalid Parsing response format: Parse error: {e}. Response: {resp}"
                    ),
                )
            })?;
        Ok(data)
    }


    /// Get User Data Timestamp.
    ///
    /// **Endpoint:** `GET /exchange/user_data_timestamp`
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_user_data_timestamp(
        &self,
    ) -> Result<GetUserDataTimestampResponse, KalshiError> {
        let resp = self.unauthenticated_get(GET_USER_DATA_TIMESTAMP).await?;
        let data: GetUserDataTimestampResponse = serde_json::from_str(&resp)
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
