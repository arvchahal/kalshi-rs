use reqwest::{Client, Response, Url};
use serde_json::Value;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const KALSHI: &str = "https://api.elections.kalshi.com/trade-api/v2/markets";

    let resp = send(KALSHI).await?;
    let json: Value = resp.error_for_status()?.json().await?;
    let pretty = serde_json::to_string_pretty(&json)?;

    fs::write("markets_pretty.json", pretty).await?;
    println!("saved: markets_pretty.json");
    Ok(())
}

async fn send(url: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = Url::parse(url)?;                 // ParseError handled by Box<dyn Error>
    let resp = client.get(url).send().await?;   // reqwest::Error also fits Box<dyn Error>
    Ok(resp)
}
