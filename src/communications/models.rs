use derive_more::Display;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCommunicationsIDResponse{
    pub communcation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRFQResponse{

}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct CreateQuoteResponse{
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQuoteRequest{
    rfq_id:String, 
    yes_bid: String,
    no_bid: String, 
    rest_remainder: bool
}