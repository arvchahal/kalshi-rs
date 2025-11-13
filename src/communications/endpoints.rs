use crate::client::KalshiClient;
use crate::communications::models::{
    Accept, AcceptQuoteResponse, ConfirmQuoteResponse, CreateQuoteRequest,
    CreateQuoteResponse, DeleteQuoteResponse, DeleteRFQResponse,
    GetCommunicationsIDResponse, GetRFQResponse,
};
use crate::errors::KalshiError;
const ACCEPT_QUOTE: &str = "/trade-api/v2/communications/quotes/{quote_id}/accept";
const CONFIRM_QUOTE: &str = "/trade-api/v2/communications/quotes/{quote_id}/confirm";
const CREATE_QUOTE: &str = "/trade-api/v2/communications/quotes";
const GET_RFQ: &str = "/trade-api/v2/communications/rfqs/{}";
const DELETE_RFQ: &str = "/trade-api/v2/communications/rfqs/{}";
const GET_QUOTES: &str = "/trade-api/v2/communications/quotes";
const DELETE_QUOTE: &str = "/trade-api/v2/communications/quotes/{}";
const GET_QUOTE: &str = "/trade-api/v2/communications/quotes/{}";
const GET_COMMUNICATIONS_ID: &str = "/trade-api/v2/communications/id";
const GET_RFQS: &str = "/trade-api/v2/communications/rfqs";
const CREATE_RFQ: &str = "/trade-api/v2/communications/rfqs";
impl KalshiClient {
    pub async fn accept_quote(
        &self,
        quote_id: &str,
        accepted_side: &str,
    ) -> Result<AcceptQuoteResponse, KalshiError> {
        let accept: Accept = Accept::from_str(accepted_side).unwrap();
        let url = ACCEPT_QUOTE.replace("{}", quote_id);
        let resp = self.authenticated_get::<Accept>(&url, Some(&accept)).await?;
        let data: AcceptQuoteResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Parse error: {e}")))?;
        Ok(data)
    }
    pub async fn confirm_quote(
        &self,
        quote_id: &str,
    ) -> Result<ConfirmQuoteResponse, KalshiError> {
        let url = CONFIRM_QUOTE.replace("{}", quote_id);
        let resp = self.authenticated_get::<str>(&url, None).await?;
        let data: ConfirmQuoteResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Parse error: {e}")))?;
        Ok(data)
    }
    pub async fn create_quote(
        &self,
        body: CreateQuoteRequest,
    ) -> Result<CreateQuoteResponse, KalshiError> {
        let resp = self
            .authenticated_get::<CreateQuoteRequest>(CREATE_QUOTE, Some(&body))
            .await?;
        let data: CreateQuoteResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Parse error: {e}")))?;
        Ok(data)
    }
    pub async fn get_rfq(&self, rfq_id: &str) -> Result<GetRFQResponse, KalshiError> {
        let url = GET_RFQ.replace("{}", rfq_id);
        let resp = self.authenticated_get::<str>(&url, None).await?;
        let data: GetRFQResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Parse error: {e}")))?;
        Ok(data)
    }
    pub async fn get_quotes() {}
    pub async fn delete_quote(
        &self,
        quote_id: &str,
    ) -> Result<DeleteQuoteResponse, KalshiError> {
        let url = DELETE_QUOTE.replace("{}", quote_id);
        let (_, resp) = self.authenticated_delete::<str>(&url, None).await?;
        let data: DeleteQuoteResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::ParseError(e))?;
        Ok(data)
    }
    pub async fn delete_rfq(
        &self,
        rfq_id: &str,
    ) -> Result<DeleteRFQResponse, KalshiError> {
        let url = DELETE_RFQ.replace("{}", rfq_id);
        let (_, resp) = self.authenticated_delete::<str>(&url, None).await?;
        let data: DeleteRFQResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::ParseError(e))?;
        Ok(data)
    }
    pub async fn get_communications_id(
        &self,
    ) -> Result<GetCommunicationsIDResponse, KalshiError> {
        let resp = self.authenticated_get::<str>(GET_COMMUNICATIONS_ID, None).await?;
        let data: GetCommunicationsIDResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Parse error: {e}")))?;
        Ok(data)
    }
    pub async fn get_rfqs() {}
    pub async fn get_quote() {}
    pub async fn create_rfq() {}
}
