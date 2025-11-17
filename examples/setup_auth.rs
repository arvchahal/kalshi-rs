use kalshi_rust_sdk::KalshiClient;
use kalshi_rust_sdk::auth::Account;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key_id = std::env::var("KALSHI_API_KEY_ID")
        .expect("KALSHI_API_KEY_ID must be set");
    let account = Account::from_file("kalshi_private.pem", api_key_id)?;
    let client = KalshiClient::new(account);
    let api_key_id = "enter your api key";
    let account = Account::from_file("kalshi_private.pem", api_key_id)?;
    let client = KalshiClient::new(account);
    Ok(())
}
