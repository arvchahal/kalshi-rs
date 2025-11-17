// Get account balance, positions, fills, and orders
//
// Run with: cargo run --example setup_auth

use kalshi_rust_sdk::KalshiClient;
use kalshi_rust_sdk::auth::Account;
use kalshi_rust_sdk::portfolio::models::*;
#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>>{
    //2 main ways to set up auth

    //first is most reccoemnded 
    //first export your kalshi api key id using export KALSHI_API_KEY_ID="YOUR API KEY"
    //then provide the path to your kalshi private key can be absolute or relativate path
    let api_key_id = std::env::var("KALSHI_API_KEY_ID")
        .expect("KALSHI_API_KEY_ID must be set");
    let account = Account::from_file("kalshi_private.pem", api_key_id)?;
    let client = KalshiClient::new(account); //new initialized client you can use

    //second just keep your keyid as a string but probably don't commit it...
    let api_key_id = "enter your api key";
    let account = Account::from_file("kalshi_private.pem", api_key_id)?;
    let client = KalshiClient::new(account);//new initialized client you can use
    

    Ok(())

}