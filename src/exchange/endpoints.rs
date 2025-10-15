use std::vec;

use crate::client::KalshiClient;
use crate::errors::KalshiError;
use crate::exchange::models::{GetExchangeAnnouncementsResponse, GetExchangeScheduleResponse};
const GET_EXCHANGE_ANNOUNCEMENTS: &str ="/trade-api/v2/exchange/announcements"; //no auth GET
const GET_EXCHANGE_SCHEDULE: &str ="/trade-api/v2/exchange/schedule"; //no auth GET
const GET_EXCHANGE_STATUS:&str = "/trade-api/v2/exchange/status"; //no auth GET
const GET_USER_DATA_TIMESTAMP: &str = "/trade-api/v2/exchange/user_data_timestamp"; //no auth GET


impl KalshiClient{
    pub async fn get_exchange_announcements(&self) -> Result<GetExchangeAnnouncementsResponse,KalshiError>{
        let resp = self.unauthenticated_get(GET_EXCHANGE_ANNOUNCEMENTS).await?;
        if resp.trim().is_empty(){
            return Ok(GetExchangeAnnouncementsResponse{announcements:vec![]})
        }
        let data:GetExchangeAnnouncementsResponse = serde_json::from_str(&resp).map_err(|e| KalshiError::Other(format!("Parse error: {e}. Response: {resp}")))?;
        Ok(data)
    }

    pub async fn get_exchange_schedule(&self)->Result<GetExchangeScheduleResponse,KalshiError>{
        let resp = self.unauthenticated_get(GET_EXCHANGE_SCHEDULE).await?;
        let data:GetExchangeScheduleResponse = serde_json::from_str(&resp).map_err(|e| KalshiError::Other(format!("Parse error: {e}. Response: {resp}")))?;
        Ok(data)
    }
}