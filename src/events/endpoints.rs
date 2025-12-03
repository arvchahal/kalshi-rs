//! Events module endpoints.
//!
//! This module implements API endpoints for events operations.

use crate::client::KalshiClient;
use crate::errors::KalshiError;
use crate::events::models::{
    EventsQuery, GetEventMetadataResponse, GetEventResponse, GetEventsResponse,
};


const GET_EVENTS: &str = "/trade-api/v2/events";


const GET_EVENT: &str = "/trade-api/v2/events/{}";


const GET_EVENT_META: &str = "/trade-api/v2/events/{}/metadata";


impl KalshiClient {
    /// get
    /// Returns all available events (optionally filterable by limit, cursor, etc.)
    pub async fn get_all_events(
        &self,
        params: &EventsQuery,
    ) -> Result<GetEventsResponse, KalshiError> {
        let query = serde_urlencoded::to_string(params)
            .map_err(|e| KalshiError::Other(
                format!("Failed to serialize params: {}", e),
            ))?;
        let url = if query.is_empty() {
            GET_EVENTS.to_string()
        } else {
            format!("{}?{}", GET_EVENTS, query)
        };
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetEventsResponse = serde_json::from_str(&resp)
            .map_err(|e| {
                KalshiError::Other(
                    format!("Invalid parse format: Parse error: {e}. Response: {resp}"),
                )
            })?;
        Ok(data)
    }
    /// GET /trade-api/v2/events/{ticker}
    /// Returns the specified event and its markets
    pub async fn get_event(
        &self,
        event_ticker: &str,
    ) -> Result<GetEventResponse, KalshiError> {
        let url = GET_EVENT.replace("{}", event_ticker);
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetEventResponse = serde_json::from_str(&resp)
            .map_err(|e| {
                KalshiError::Other(
                    format!("Invalid parse format: Parse error: {e}. Response: {resp}"),
                )
            })?;
        Ok(data)
    }
    /// get /trade-api/v2/events/{ticker}/metadata
    /// Returns metadata such as image URL, competition, and settlement sources
    pub async fn get_event_metadata(
        &self,
        event_ticker: &str,
    ) -> Result<GetEventMetadataResponse, KalshiError> {
        let url = GET_EVENT_META.replace("{}", event_ticker);
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetEventMetadataResponse = serde_json::from_str(&resp)
            .map_err(|e| {
                KalshiError::Other(
                    format!("Invalid parse format: Parse error: {e}. Response: {resp}"),
                )
            })?;
        Ok(data)
    }
}
