use crate::auth::Account;
use crate::errors::KalshiError;
use crate::helpers;
use reqwest::{Client, StatusCode};
const KALSHI_API: &str = "https://api.elections.kalshi.com";
pub struct KalshiClient {
    pub(crate) http_client: Client,
    pub(crate) account: Account,
    pub(crate) base_url: String,
}
impl KalshiClient {
    pub fn new(user: Account) -> KalshiClient {
        KalshiClient {
            http_client: Client::new(),
            account: user,
            base_url: KALSHI_API.to_string(),
        }
    }
    pub fn new_with_config(
        user: Account,
        configuration: Option<String>,
    ) -> KalshiClient {
        KalshiClient {
            http_client: Client::new(),
            account: user,
            base_url: configuration.unwrap_or_else(|| KALSHI_API.to_string()),
        }
    }
    /// Wrapper for authenticated GET requests
    pub async fn authenticated_get<T>(
        &self,
        path: &str,
        body: Option<&T>,
    ) -> Result<String, KalshiError>
    where
        T: serde::Serialize + ?Sized,
    {
        helpers::authenticated_get(
                &self.http_client,
                &self.base_url,
                &self.account,
                path,
                body,
            )
            .await
    }
    /// Wrapper for authenticated POST requests
    pub async fn authenticated_post<T>(
        &self,
        path: &str,
        json_body: Option<&T>,
    ) -> Result<String, KalshiError>
    where
        T: serde::Serialize + ?Sized,
    {
        helpers::authenticated_post(
                &self.http_client,
                &self.base_url,
                &self.account,
                path,
                json_body,
            )
            .await
    }
    /// Wrapper for authenticated DELETE requests
    pub async fn authenticated_delete<T>(
        &self,
        path: &str,
        body: Option<&T>,
    ) -> Result<(StatusCode, String), KalshiError>
    where
        T: serde::Serialize + ?Sized,
    {
        helpers::authenticated_delete(
                &self.http_client,
                &self.base_url,
                &self.account,
                path,
                body,
            )
            .await
    }
    /// Wrapper for unauthenticated GET requests
    pub async fn unauthenticated_get(&self, path: &str) -> Result<String, KalshiError> {
        helpers::unauthenticated_get(&self.http_client, &self.base_url, path).await
    }
    /// Wrapper for authenticated put requests
    pub async fn authenticated_put<T>(
        &self,
        path: &str,
        json_body: Option<&T>,
    ) -> Result<(StatusCode, String), KalshiError>
    where
        T: serde::Serialize + ?Sized,
    {
        helpers::authenticated_put(
                &self.http_client,
                &self.base_url,
                &self.account,
                path,
                json_body,
            )
            .await
    }
}
