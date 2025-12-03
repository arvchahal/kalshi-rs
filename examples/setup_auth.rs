use kalshi_rs::auth::Account;
#[tokio::main]
///first export your kalshi api key id using export KALSHI_API_KEY_ID="YOUR API KEY"
///then provide the path to your kalshi private key can be absolute or relativate path
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key_id = std::env::var("KALSHI_API_KEY_ID").expect("KALSHI_API_KEY_ID must be set");
    let account = Account::from_file("kalshi_private.pem", api_key_id)?;
    println!("{:?}", account);
    Ok(())
}
