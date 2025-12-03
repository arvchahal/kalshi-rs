use kalshi_rs::auth::Account;
use kalshi_rs::markets::models::MarketsQuery;
use kalshi_rs::KalshiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup authentication... change the kalshi_private.pem to your path to the downloaded file....
    //or use an enviroment variable to hold the path
    let api_key_id = std::env::var("KALSHI_API_KEY_ID")
        .expect("KALSHI_API_KEY_ID environment variable must be set");
    //
    let account = Account::from_file("kalshi_private.pem", api_key_id)?;
    let client = KalshiClient::new(account);

    // Fetch active markets
    println!("Fetching active markets from Kalshi...\n");
    let params = MarketsQuery {
        limit: Some(5),
        status: Some("active".to_string()),
        ..Default::default()
    };
    let markets = client.get_all_markets(&params).await?;

    // Display market information
    println!("Found {} active markets:\n", markets.markets.len());
    for (i, market) in markets.markets.iter().enumerate() {
        println!("{}. {}", i + 1, market.ticker);
        println!("   Title: {}", market.title);
        println!("   Subtitle: {}", market.subtitle);
        println!("   Status: {}", market.status);
        println!();
    }

    // Fetch account balance
    println!("Fetching account balance...");
    let balance = client.get_balance().await?;
    println!("Account balance: ${}", balance.balance);

    println!("\n✓ SDK is working correctly!");

    Ok(())
}
