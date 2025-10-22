use kalshi_rust_sdk::KalshiClient;
use reqwest::{Client, Response, Url};
use serde_json::Value;
use tokio::fs;
use kalshi_rust_sdk::auth::auth_loader::load_auth_from_file;
use kalshi_rust_sdk::auth::Account;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // const KALSHI: &str = "https://api.elections.kalshi.com/trade-api/v2/markets";
    let x = load_auth()?;
    let t = KalshiClient::new(x);
    let api_ = t.get_api_keys().await;

    println!("{:?}",api_?);
    // let announcements = t.get_exchange_announcements().await;
    // println!("{}",announcements?);
    // let sched = t.get_exchange_schedule().await?;
    // println!("{}",sched);
    // let status = t.get_exchange_status().await?;
    // println!("{}",status);
    // let user_data = t.get_user_data_timestamp().await?;
    // println!("{}",user_data);
    // let market_data = t.get_market().await?;
    // println!("{}",market_data);

    
    Ok(())
}

async fn send(url: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = Url::parse(url)?;                //ParseError handled by Box<dyn Error>
    let resp = client.get(url).send().await?;   //reqwest::Error also fits Box<dyn Error>
    Ok(resp)
}

fn load_auth() -> Result<Account, Box<dyn std::error::Error>> {
    let _account = load_auth_from_file()?;
    Ok(_account)
}