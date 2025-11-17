use kalshi_rs::KalshiClient;
use kalshi_rs::auth::Account;
use kalshi_rs::markets::models::*;


#[tokio::main]
/// Browse and filter markets using MarketsQuery
///
/// Run with: cargo run --example browse_markets
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key_id = std::env::var("KALSHI_API_KEY_ID")
        .expect("KALSHI_API_KEY_ID must be set");
    let account = Account::from_file("kalshi_private.pem", api_key_id)?;
    let client = KalshiClient::new(account);


    println!("1. Getting active markets...");
    let active_params = MarketsQuery {
        limit: Some(5),
        cursor: None,
        event_ticker: None,
        series_ticker: None,
        max_close_ts: None,
        min_close_ts: None,
        status: Some("active".to_string()),
        tickers: None,
    };


    let active_markets = client.get_all_markets(&active_params).await?;
    println!("   Found {} active markets", active_markets.markets.len());
    for market in active_markets.markets.iter().take(3) {
        println!("   - {}: {}", market.ticker, market.title);
    }


    if !active_markets.markets.is_empty() {
        let first_event = &active_markets.markets[0].event_ticker;
        println!("\n2. Getting markets for event: {}", first_event);
        let event_params = MarketsQuery {
            limit: Some(10),
            cursor: None,
            event_ticker: Some(first_event.clone()),
            series_ticker: None,
            max_close_ts: None,
            min_close_ts: None,
            status: None,
            tickers: None,
        };


        let event_markets = client.get_all_markets(&event_params).await?;
        println!("   Found {} markets in this event", event_markets.markets.len());
        for market in event_markets.markets.iter() {
            println!("   - {} (status: {})", market.ticker, market.status);
        }
    }


    println!("\n3. Getting markets closing in the next 7 days...");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64;
    let seven_days_later = now + (7 * 24 * 60 * 60);
    let time_params = MarketsQuery {
        limit: Some(5),
        cursor: None,
        event_ticker: None,
        series_ticker: None,
        max_close_ts: Some(seven_days_later),
        min_close_ts: Some(now),
        status: Some("active".to_string()),
        tickers: None,
    };

    let time_filtered = client.get_all_markets(&time_params).await?;
    println!("   Found {} markets closing soon", time_filtered.markets.len());
    for market in time_filtered.markets.iter().take(5) {
        println!("   - {}: closes at {}", market.ticker, market.close_time);
    }


    println!("\n4. Demonstrating pagination...");
    let mut all_markets = Vec::new();
    let mut cursor = None;
    let mut page = 1;

    loop {
        let page_params = MarketsQuery {
            limit: Some(3),
            cursor: cursor.clone(),
            event_ticker: None,
            series_ticker: None,
            max_close_ts: None,
            min_close_ts: None,
            status: Some("active".to_string()),
            tickers: None,
        };

        let response = client.get_all_markets(&page_params).await?;
        let count = response.markets.len();
        all_markets.extend(response.markets);
        println!("   Page {}: fetched {} markets", page, count);

        if response.cursor.is_some() && page < 3 {
            cursor = response.cursor;
            page += 1;
        } else {
            break;
        }
    }


    println!("   Total markets fetched: {}", all_markets.len());
    println!("\nMarket browsing example finished");
    Ok(())
}
