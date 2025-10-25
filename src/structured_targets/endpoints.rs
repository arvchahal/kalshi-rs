


const STRUCTURED_TARGETS: &str = "/trade-api/v2/structured_targets";
const STRUCTURED_TARGET: &str =  "/trade-api/v2/structured_targets/{}"; // no auth get re[place {} wiht specific event ticker
use crate::{helpers::build_url_with_query, KalshiClient};
use crate::errors::KalshiError;
use crate::structured_targets::models::{GetStructuredTargetsResponse,GetStructuredTargetResponse};

impl KalshiClient{
    pub async fn get_all_structured_targets(&self, limit: Option<u64>)-> Result<GetStructuredTargetsResponse, KalshiError>{
        let _limit = limit.unwrap_or(100);
        let url = build_url_with_query(STRUCTURED_TARGETS.to_string(),&_limit);
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetStructuredTargetsResponse = serde_json::from_str(&resp).map_err( |e| KalshiError::Other(format!("Parse error: {e}. Response: {resp}")))?;
        Ok(data)
    }

    pub async fn get_structured_target(&self, structured_target_id:&str)-> Result<GetStructuredTargetResponse, KalshiError>{
        let url: String = STRUCTURED_TARGET.replace("{}", structured_target_id);
        let resp: String = self.unauthenticated_get(&url).await?;
        let data: GetStructuredTargetResponse = serde_json::from_str(&resp).map_err( |e| KalshiError::Other(format!("Parse error: {e}. Response: {resp}")))?;
        Ok(data)
    }
}