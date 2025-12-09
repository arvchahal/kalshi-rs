use kalshi_rs::KalshiWebsocketClient;
use kalshi_rs::auth::Account;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {   
    // create account
    let api_key_id = std::env::var("KALSHI_API_KEY_ID")
        .expect("KALSHI_API_KEY_ID must be set");
    let account = Account::from_file("kalshi_private.pem", api_key_id)?;

    // create ws client
    let client = KalshiWebsocketClient::new(account);
    
    // create ws connection
    client.connect().await?;

    // subscribe to orderbook delta channel for ticker
    let channels: Vec<&str> = vec!["orderbook_delta"];
    let tickers: Vec<&str> = vec!["KXBTCD-25DEC0916-T92999.99"];
    client.subscribe(
        channels,
        tickers,
    ).await?;

    // print messages as they arrive
    loop {
        let msg = client.next_message().await;
        println!("{msg:?}");
    }
}