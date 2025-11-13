use kalshi_rust_sdk::KalshiClient;
use kalshi_rust_sdk::auth::Account;
use kalshi_rust_sdk::auth::auth_loader::load_auth_from_file;
use kalshi_rust_sdk::portfolio::models::CreateOrderGroupRequest;
use kalshi_rust_sdk::series::models::GetSeriesResponse;
use kalshi_rust_sdk::errors::KalshiError;
use reqwest::{Client, Response, Url};
use serde_json::Value;
use tokio::fs;
#[tokio::main]
async fn main() {
    const KALSHI: &str = "https://api.elections.kalshi.com/trade-api/v2/markets";
    let x = load_auth().unwrap();
    let t: KalshiClient = KalshiClient::new(x);
}
async fn send(url: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = Url::parse(url)?;
    let resp = client.get(url).send().await?;
    Ok(resp)
}
fn load_auth() -> Result<Account, Box<dyn std::error::Error>> {
    let _account = load_auth_from_file()?;
    Ok(_account)
}


