use crate::client::KalshiClient;
use crate::api_keys::models::{ApiKey, DeleteApiKeyResponse, CreateApiKeyResponse, ListApiKeysResponse};
use crate::errors::KalshiError;
use serde_json::json;


const ACCEPT_QUOTE: &str = "/trade-api/v2/communications/quotes/{quote_id}/accept";
const CONFIRM_QUOTE: &str = "/trade-api/v2/communications/quotes/{quote_id}/confirm";
const CREATE_QUOTE: &str ="/trade-api/v2/communications/quotes";
const GET_RFQ: &str = "/trade-api/v2/communications/rfqs/{}";
const DELETE_RFQ:&str = "/trade-api/v2/communications/rfqs/{rfq_id}";
const GET_QUOTES: &str = "/trade-api/v2/communications/quotes";
const DELETE_QUOTE: &str ="/trade-api/v2/communications/quotes/{}";
const GET_QUOTE: &str = "/trade-api/v2/communications/quotes/{}";
const GET_COMMUNICATIONS_ID: &str ="/trade-api/v2/communications/id";
const GET_RFQS: &str ="/trade-api/v2/communications/rfqs";
const CREATE_RFQ: &str ="/trade-api/v2/communications/rfqs";

pub async fn accept_quote(){

}
pub async fn confirm_quote(){
    
}
pub async fn create_quote(){
    
}
pub async fn get_rfq(){
    
}
pub async fn get_quotes(){
    
}
pub async fn delete_quote(){
    
}
pub async fn delete_rfq(){
    
}
pub async fn get_communications_id(){
    
}
pub async fn get_rfqs(){
    
}
pub async fn get_quote(){
    
}
pub async fn create_rfq(){
    
}
