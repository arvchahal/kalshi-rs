use kalshi_rust_sdk::KalshiClient;
use kalshi_rust_sdk::auth::Account;
use kalshi_rust_sdk::auth::auth_loader::load_auth_from_file;
use kalshi_rust_sdk::portfolio::models::CreateOrderGroupRequest;
use reqwest::{Client, Response, Url};
use serde_json::Value;
use tokio::fs;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const KALSHI: &str = "https://api.elections.kalshi.com/trade-api/v2/markets";
    let x = load_auth()?;
    let t = KalshiClient::new(x);
    let og = CreateOrderGroupRequest {
        contracts_limit: 1,
    };
    let s = t.create_order_group(&og).await?;
    let g = t.get_order_group(&s.order_group_id).await?;
    println!("{:?}", g);
    Ok(())
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
