use reqwest::{Client, Response, Url};
use serde_json::Value;
use tokio::fs;
use kalshi_rust_sdk::auth::auth_loader::load_auth_from_file;
use kalshi_rust_sdk::auth::Account;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // const KALSHI: &str = "https://api.elections.kalshi.com/trade-api/v2/markets";

    // let resp = send(KALSHI).await?;
    // let json: Value = resp.error_for_status()?.json().await?;
    // let pretty = serde_json::to_string_pretty(&json)?;

    // fs::write("markets_pretty.json", pretty).await?;
    // println!("saved: markets_pretty.json");
    let x = load_auth()?;
    println!("{}",x);

    Ok(())
}

async fn send(url: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = Url::parse(url)?;                 // ParseError handled by Box<dyn Error>
    let resp = client.get(url).send().await?;   // reqwest::Error also fits Box<dyn Error>
    Ok(resp)
}

fn load_auth() -> Result<Account, Box<dyn std::error::Error>> {
    let _account = load_auth_from_file()?;
    println!("hi {:?}",_account);
    Ok(_account)
}