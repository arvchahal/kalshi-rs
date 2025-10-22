use crate::client::KalshiClient;
use crate::errors::KalshiError;
use crate::milestones::models::{GetMilestoneResponse,GetMilestonesResponse};
const GET_MILESTONE: &str = "/trade-api/v2/milestones/{}"; //unauth get replace {} with the milestone id

impl KalshiClient{
    pub async fn get_milestone(&self, id:&str)-> Result<GetMilestoneResponse, KalshiError>{
        let url: &str = GET_MILESTONE.replace("{}",id);
        let resp: String = self.unauthenticated_get(url);
        let data: GetMilestoneResponse= serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Invalid Parsing response format: Parse error: {e}. Response: {resp}")))?;
        Ok(data)
    }
}