
use crate::auth::Account;
use crate::errors::KalshiError;
use crate::helpers;
use reqwest::{Client, StatusCode};

/*
Main entry point for the entire sdk implemented in multiple parts across different crates to promote
modularity..

Each implementation follows the python sdk api classes
https://docs.kalshi.com/python-sdk/api/PortfolioApi
Each folder in has an ###endpoints.rs#### that has all of the methods to hit and get data from the endpoints
and then parse the json into the native structs
then there is a ###models.rs### that contains the native structs and their implementations
Finally there is a mod.rs that exposes the crates

*/


const KALSHI_API: &str = "https://api.elections.kalshi.com";

pub struct KalshiClient{
    pub(crate) http_client: Client,
    pub(crate) account: Account,
    pub(crate) base_url: String
}

impl KalshiClient{
    pub fn new(user:Account) -> KalshiClient{
        KalshiClient{
            http_client: Client::new(),
            account:user,
            base_url: KALSHI_API.to_string(),
        }
    }

    /// Wrapper for authenticated GET requests
    pub async fn authenticated_get(&self, path: &str) -> Result<String, KalshiError> {
        helpers::authenticated_get(&self.http_client, &self.base_url, &self.account, path).await
    }

    /// Wrapper for authenticated POST requests
    pub async fn authenticated_post<T>(&self, path: &str, json_body: Option<&T>) -> Result<String, KalshiError>
    where
        T: serde::Serialize + ?Sized,
    {
        helpers::authenticated_post(&self.http_client, &self.base_url, &self.account, path, json_body).await
    }

    /// Wrapper for authenticated DELETE requests
    pub async fn authenticated_delete(&self, path: &str) -> Result<(StatusCode,String), KalshiError> {
        helpers::authenticated_delete(&self.http_client, &self.base_url, &self.account, path).await
    }

    /// Wrapper for unauthenticated GET requests
    pub async fn unauthenticated_get(&self, path: &str) -> Result<String, KalshiError> {
        helpers::unauthenticated_get(&self.http_client, &self.base_url, path).await
    }
}