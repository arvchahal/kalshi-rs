use reqwest::Client;
use crate::auth::Account;
use crate::errors::KalshiError;
use crate::helpers;

const KALSHI_API: &str = "https://api.elections.kalshi.com";

pub struct KalshiClient{
    pub(crate) http_client: Client,
    pub(crate) account: Account,
    pub(crate) base_url: String
}

impl KalshiClient{
    pub fn new(user: Account) -> KalshiClient{
        KalshiClient{
            http_client: Client::new(),
            account: user,
            base_url: KALSHI_API.to_string(),
        }
    }

    /// Make an unauthenticated GET request (for public endpoints)
    pub(crate) async fn get(&self, path: &str) -> Result<String, KalshiError> {
        helpers::unauthenticated_get(&self.http_client, &self.base_url, path).await
    }

    /// Make an authenticated GET request
    pub(crate) async fn authenticated_get(&self, path: &str) -> Result<String, KalshiError> {
        helpers::authenticated_get(&self.http_client, &self.base_url, &self.account, path).await
    }

    /// Make an authenticated POST request
    pub(crate) async fn authenticated_post(
        &self,
        path: &str,
        json_body: Option<&impl serde::Serialize>,
    ) -> Result<String, KalshiError> {
        helpers::authenticated_post(&self.http_client, &self.base_url, &self.account, path, json_body).await
    }

    /// Make an authenticated DELETE request
    pub(crate) async fn authenticated_delete(&self, path: &str) -> Result<String, KalshiError> {
        helpers::authenticated_delete(&self.http_client, &self.base_url, &self.account, path).await
    }
}