use derive_more::Display;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCommunicationsIDResponse{
    pub communcation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRFQResponse{

}