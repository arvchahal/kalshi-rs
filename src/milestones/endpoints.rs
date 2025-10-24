use crate::client::KalshiClient;
use crate::errors::KalshiError;
use crate::helpers::build_url_with_query;
use crate::milestones::models::{GetMilestoneResponse,GetMilestonesResponse};
const GET_MILESTONE: &str = "/trade-api/v2/milestones/{}"; //unauth get replace {} with the milestone id
const GET_MILESTONES:&str = "/trade-api/v2/milestones";
impl KalshiClient{
    pub async fn get_milestone(&self, id: &str) -> Result<GetMilestoneResponse, KalshiError>{
        let url = GET_MILESTONE.replace("{}", id);
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetMilestoneResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Invalid Parsing response format: Parse error: {e}. Response: {resp}")))?;
        Ok(data)
    }
    pub async fn get_milestones(&self, limit: Option<u32>)-> Result<GetMilestonesResponse,KalshiError>{

        let mut _true_limit = limit.unwrap_or(100);
        let url = build_url_with_query(GET_MILESTONES.to_string(), &_true_limit);
        let resp = self.unauthenticated_get(&url).await?;
        let data: GetMilestonesResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Invalid Parsing response format: Parse error: {e}. Response: {resp}")))?;
        Ok(data)

    }
}