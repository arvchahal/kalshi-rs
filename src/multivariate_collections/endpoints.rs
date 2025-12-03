//! Multivariate_collections module endpoints.
//!
//! This module implements API endpoints for multivariate_collections operations.

use crate::client::KalshiClient;
use crate::errors::KalshiError;
use crate::multivariate_collections::models::{
    GetMultivariateEventCollectionResponse, GetMultivariateEventCollectionsResponse,
};
const GET_MVE_COL: &str = "/trade-api/v2/multivariate_event_collections/{}";
const GET_MVE_COLS: &str = "/trade-api/v2/multivariate_event_collections/";


impl KalshiClient {
    /// Get Multivariate Event Collection.
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_multivariate_event_collection(
        &self,
        collection_ticker: &str,
    ) -> Result<GetMultivariateEventCollectionResponse, KalshiError> {
        let url = GET_MVE_COL.replace("{}", collection_ticker);
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetMultivariateEventCollectionResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Parse error: {e}. Response: {resp}"),
            ))?;
        Ok(data)
    }


    
    /// Get Multivariate Event Collections.
    ///
    /// # Returns
    /// Result with response data or error
    pub async fn get_multivariate_event_collections(
        &self,
    ) -> Result<GetMultivariateEventCollectionsResponse, KalshiError> {
        let url = GET_MVE_COLS.to_string();
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetMultivariateEventCollectionsResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(
                format!("Parse error: {e}. Response: {resp}"),
            ))?;
        Ok(data)
    }
}
