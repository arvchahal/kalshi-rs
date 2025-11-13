use crate::client::KalshiClient;
use crate::communications::models::{
    GetCommunicationsIDResponse,GetRFQResponse, CreateQuoteRequest, CreateQuoteResponse

};
use crate::errors::KalshiError;
use serde_json::json;


const ACCEPT_QUOTE: &str = "/trade-api/v2/communications/quotes/{quote_id}/accept";
const CONFIRM_QUOTE: &str = "/trade-api/v2/communications/quotes/{quote_id}/confirm";
const CREATE_QUOTE: &str ="/trade-api/v2/communications/quotes";
const GET_RFQ: &str = "/trade-api/v2/communications/rfqs/{}";
const DELETE_RFQ:&str = "/trade-api/v2/communications/rfqs/{}";
const GET_QUOTES: &str = "/trade-api/v2/communications/quotes";
const DELETE_QUOTE: &str ="/trade-api/v2/communications/quotes/{}";
const GET_QUOTE: &str = "/trade-api/v2/communications/quotes/{}";
const GET_COMMUNICATIONS_ID: &str ="/trade-api/v2/communications/id";
const GET_RFQS: &str ="/trade-api/v2/communications/rfqs";
const CREATE_RFQ: &str ="/trade-api/v2/communications/rfqs";

impl KalshiClient{
    pub async fn accept_quote(){

    }
    pub async fn confirm_quote(){
        
    }
    pub async fn create_quote(&self, body:CreateQuoteRequest )-> Result<CreateQuoteResponse, KalshiError>{
        let resp= self.authenticated_get::<CreateQuoteRequest>(CREATE_QUOTE, Some(&body)).await?;
        let data: CreateQuoteResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Parse error: {e}")))?;
        Ok(data)

        
    }
    pub async fn get_rfq(&self, rfq_id:&str)->Result<GetRFQResponse,KalshiError>{
        let url = GET_RFQ.replace("{}", rfq_id);
        let resp = self.authenticated_get::<str>(&url, None).await?;
        let data: GetRFQResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Parse error: {e}")))?;
        Ok(data)
        
    }
    pub async fn get_quotes(){
        
    }
    pub async fn delete_quote(){
        
    }
    pub async fn delete_rfq(){
        
    }
    pub async fn get_communications_id(&self)->Result<GetCommunicationsIDResponse, KalshiError>{
        let resp = self.authenticated_get::<str>(GET_COMMUNICATIONS_ID, None).await?;
        let data: GetCommunicationsIDResponse = serde_json::from_str(&resp)
            .map_err(|e| KalshiError::Other(format!("Parse error: {e}")))?;
        Ok(data)
    }
    pub async fn get_rfqs(){
        
    }
    pub async fn get_quote(){
        
    }
    pub async fn create_rfq(){
        
    }
}