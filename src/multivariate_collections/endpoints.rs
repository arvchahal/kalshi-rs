use crate::client::KalshiClient;
use crate::errors::KalshiError;
use crate::helpers::build_url_with_query;
use crate::multivariate_collections::models::{GetMultivariateEventCollectionResponse,
    GetMultivariateEventCollectionsResponse//, LookupBundleResponse
};

const GET_MVE_COL: &str = "/trade-api/v2/multivariate_event_collections/{}";//replace {} with collection ticker no auth get
const GET_MVE_COLS: &str = "/trade-api/v2/multivariate_event_collections/";
// const LOOKUP_MVE_BUNDLE:&str = "/trade-api/v2/multivariate_event_collections//lookup";

impl KalshiClient{
    pub async fn get_multivariate_event_collection(
        &self,
        collection_ticker: &str,
    ) -> Result<GetMultivariateEventCollectionResponse, KalshiError> {
        let url = GET_MVE_COL.replace("{}", collection_ticker);
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetMultivariateEventCollectionResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!(
                "Parse error: {e}. Response: {resp}"
            )))?;
        Ok(data)
    }

    pub async fn get_multivariate_event_collections(
        &self,
    ) -> Result<GetMultivariateEventCollectionsResponse, KalshiError> {
        let url = GET_MVE_COLS.to_string();
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetMultivariateEventCollectionsResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!(
                "Parse error: {e}. Response: {resp}"
            )))?;
        Ok(data)
    }

    // pub async fn lookup_multivariate_event_collection_bundle()-> Result<LookupBundleResponse, KalshiError>{

    // }



}