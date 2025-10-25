


use crate::client::KalshiClient;
use crate::errors::KalshiError;
use crate::helpers::build_url_with_query;
use crate::series::models::{GetSeriesListResponse,GetSeriesResponse};

const GET_SERIES_LIST: &str = "/trade-api/v2/series";
const GET_SERIES_TICKER: &str = "/trade-api/v2/series/{}";//replace with ticker


impl KalshiClient {
    pub async fn get_all_series(
        &self,
        limit: Option<u16>,
        cursor: Option<&str>,
    ) -> Result<GetSeriesListResponse, KalshiError> {
        let mut params = std::collections::HashMap::new();
        if let Some(l) = limit {
            params.insert("limit", l.to_string());
        }
        if let Some(c) = cursor {
            params.insert("cursor", c.to_string());
        }

        let url = build_url_with_query(GET_SERIES_LIST.to_string(), &params);
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetSeriesListResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Parse error: {e}. Response: {resp}")))?;
        Ok(data)
    }

    pub async fn get_series_by_ticker(
        &self,
        ticker: &str,
    ) -> Result<GetSeriesResponse, KalshiError> {
        let url = GET_SERIES_TICKER.replace("{}", ticker);
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetSeriesResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Parse error: {e}. Response: {resp}")))?;
        Ok(data)
    }
}