//! Series module endpoints.
//!
//! This module implements API endpoints for series operations.

use crate::client::KalshiClient;
use crate::errors::KalshiError;
use crate::series::models::{GetSeriesListResponse, GetSeriesResponse, SeriesQuery};


const GET_SERIES_LIST: &str = "/trade-api/v2/series";
const GET_SERIES_TICKER: &str = "/trade-api/v2/series/{}";


impl KalshiClient {
    /// Get All Series.
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_all_series(
        &self,
        limit: Option<u16>,
        cursor: Option<&str>,
    ) -> Result<GetSeriesListResponse, KalshiError> {
        let params = SeriesQuery {
            limit,
            cursor: cursor.map(|s| s.to_string()),
        };
        let query = serde_urlencoded::to_string(&params)
            .map_err(|e| KalshiError::Other(
                format!("Failed to serialize params: {}", e),
            ))?;
        let url = if query.is_empty() {
            GET_SERIES_LIST.to_string()
        } else {
            format!("{}?{}", GET_SERIES_LIST, query)
        };
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetSeriesListResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Parse error: {e}.")))?;
        Ok(data)
    }
    

    
    /// Get Series By Ticker.
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_series_by_ticker(
        &self,
        ticker: &str,
    ) -> Result<GetSeriesResponse, KalshiError> {
        let url = GET_SERIES_TICKER.replace("{}", ticker);
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetSeriesResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Parse error: {e}. Response {resp}"),
            ))?;
        Ok(data)
    }
}
