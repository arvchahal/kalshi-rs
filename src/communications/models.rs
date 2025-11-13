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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteQuoteResponse{
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRFQResponse{
    pub body: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptQuoteResponse{
    pub body: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Accept {
    Yes,
    No,
}

impl Accept {
    pub fn from_str(s: &str) -> Result<Accept, String> {
        match s {
            "yes" | "Yes" | "YES" => Ok(Accept::Yes),
            "no" | "No" | "NO" => Ok(Accept::No),
            _ => Err(format!("Invalid value: {}, choose from Yes or No", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmQuoteResponse{
    pub body: Option<String>
}