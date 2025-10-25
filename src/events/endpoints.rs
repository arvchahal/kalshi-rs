use crate::client::KalshiClient;
use crate::errors::KalshiError;
// use crate::helpers::build_url_with_query;
use crate::events::models::{
    GetEventResponse, GetEventsResponse, GetEventMetadataResponse,
};

const GET_EVENTS:&str = "/trade-api/v2/events"; //no auth tickers
const GET_EVENT:&str = "/trade-api/v2/events/{}";//replace with ticker
const GET_EVENT_META: &str = "/trade-api/v2/events/{}/metadata";// no auth get for metadata about event

impl KalshiClient {
    /// get 
    /// Returns all available events (optionally filterable by limit, cursor, etc.)
    pub async fn get_all_events(
        &self,
        limit: Option<u16>,
        cursor: Option<&str>,
    ) -> Result<GetEventsResponse, KalshiError> {
        // Build query string if needed
        let mut params = vec![];
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        if let Some(c) = cursor {
            params.push(format!("cursor={}", c));
        }

        let url = if params.is_empty() {
            GET_EVENTS.to_string()
        } else {
            format!("{}?{}", GET_EVENTS, params.join("&"))
        };

        let resp = self.unauthenticated_get(&url).await?;
        let data: GetEventsResponse = serde_json::from_str(&resp).map_err(|e| {
            KalshiError::Other(format!("Invalid parse format: Parse error: {e}. Response: {resp}"))
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
        let data: GetEventResponse = serde_json::from_str(&resp).map_err(|e| {
            KalshiError::Other(format!("Invalid parse format: Parse error: {e}. Response: {resp}"))
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
        let data: GetEventMetadataResponse = serde_json::from_str(&resp).map_err(|e| {
            KalshiError::Other(format!("Invalid parse format: Parse error: {e}. Response: {resp}"))
        })?;
        Ok(data)
    }
}