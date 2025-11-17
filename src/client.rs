use crate::auth::Account;
use crate::errors::KalshiError;
use crate::helpers;
use reqwest::{Client, StatusCode};


// Kalshi API base URL for production
const KALSHI_API: &str = "https://api.elections.kalshi.com";


/// Main client for interacting with the Kalshi API.
///
/// The `KalshiClient` provides access to all Kalshi API endpoints organized by category.
/// Create a client with [`KalshiClient::new`] and use the various methods to interact with the API.
///
/// # Available Endpoint Categories
///
/// ## Markets
/// - [`get_all_markets`](KalshiClient::get_all_markets) - Retrieve market listings
/// - [`get_market`](KalshiClient::get_market) - Get individual market details
/// - [`get_trades`](KalshiClient::get_trades) - Fetch recent trades
/// - [`get_market_orderbook`](KalshiClient::get_market_orderbook) - Get market orderbook
/// - [`get_market_candlesticks`](KalshiClient::get_market_candlesticks) - Historical OHLC data
///
/// ## Portfolio
/// - [`get_balance`](KalshiClient::get_balance) - Get account balance
/// - [`get_positions`](KalshiClient::get_positions) - Get current positions
/// - [`get_orders`](KalshiClient::get_orders) - List orders
/// - [`create_order`](KalshiClient::create_order) - Place a new order
/// - [`cancel_order`](KalshiClient::cancel_order) - Cancel an order
/// - And many more order management methods...
///
/// ## Exchange
/// - [`get_exchange_status`](KalshiClient::get_exchange_status) - Exchange operational status
/// - [`get_exchange_schedule`](KalshiClient::get_exchange_schedule) - Trading schedule
///
/// ## Events & Series
/// - [`get_events`](KalshiClient::get_events) - List events
/// - [`get_event`](KalshiClient::get_event) - Get event details
/// - [`get_series`](KalshiClient::get_series) - Get series information
///
/// # Example
/// ```no_run
/// use kalshi_rust_sdk::{Account, KalshiClient};
/// use kalshi_rust_sdk::markets::models::MarketsQuery;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Create account with credentials
/// let account = Account::from_file("kalshi_private.pem", "your-api-key-id")?;
///
/// // Initialize client
/// let client = KalshiClient::new(account);
///
/// // Fetch markets
/// let markets = client.get_all_markets(&MarketsQuery {
///     limit: Some(10),
///     ..Default::default()
/// }).await?;
/// # Ok(())
/// # }
/// ```
pub struct KalshiClient {
    pub(crate) http_client: Client,
    pub(crate) account: Account,
    pub(crate) base_url: String,
}


impl KalshiClient {
    /// Create a new KalshiClient with default API endpoint
    pub fn new(user: Account) -> KalshiClient {
        KalshiClient {
            http_client: Client::new(),
            account: user,
            base_url: KALSHI_API.to_string(),
        }
    }


    /// Create a new KalshiClient with custom API endpoint
    /// Useful for testing or using different API environments
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
