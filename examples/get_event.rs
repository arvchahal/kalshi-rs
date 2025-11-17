// Get event details with all its markets
//
// Run with: cargo run --example get_event

use kalshi_rust_sdk::KalshiClient;
use kalshi_rust_sdk::auth::Account;
use kalshi_rust_sdk::events::models::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup authentication
    let api_key_id = std::env::var("KALSHI_API_KEY_ID")
        .expect("KALSHI_API_KEY_ID must be set");
    let account = Account::from_file("kalshi_private.pem", api_key_id)?;
    let client = KalshiClient::new(account);

    // 1. List all events
    println!("1. Listing recent events...");
    let events_query = EventsQuery {
        limit: Some(5),
        cursor: None,
    };

    let events = client.get_all_events(&events_query).await?;
    println!("   Found {} events\n", events.events.len());

    if events.events.is_empty() {
        println!("No events available");
        return Ok(());
    }

    // Display summary of events
    for event in events.events.iter() {
        println!("   Event: {}", event.event_ticker);
        println!("      Title: {}", event.title);
        println!("      Series: {}", event.series_ticker);
        println!();
    }

    // 2. Get detailed information for first event
    let event_ticker = &events.events[0].event_ticker;
    println!("2. Getting detailed info for event: {}\n", event_ticker);

    let event_details = client.get_event(event_ticker).await?;
    let event = &event_details.event;
    let markets = &event_details.markets;

    println!("   Event Details:");
    println!("      Ticker: {}", event.event_ticker);
    println!("      Title: {}", event.title);
    println!("      Series: {}", event.series_ticker);
    println!("      Markets: {}", markets.len());
    println!();

    // 3. Display all markets in the event
    println!("3. Markets in this event:");
    for market in markets.iter() {
        println!("   Market: {}", market.ticker);
        println!("      Title: {}", market.title);
        println!("      Status: {}", market.status);
        println!("      Yes Bid: {} cents ({})", market.yes_bid, market.yes_bid_dollars);
        println!("      Yes Ask: {} cents ({})", market.yes_ask, market.yes_ask_dollars);
        println!("      Last Price: {} cents", market.last_price);
        println!("      Volume: {}", market.volume);
        println!("      Open Interest: {}", market.open_interest);
        println!();
    }

    // 4. Get event metadata (settlement sources, etc.)
    println!("4. Getting event metadata...");
    let metadata = client.get_event_metadata(event_ticker).await?;

    if let Some(competition) = &metadata.competition {
        println!("   Competition: {}", competition);
    }
    if let Some(scope) = &metadata.competition_scope {
        println!("   Competition Scope: {}", scope);
    }

    println!("   Settlement Sources ({}):", metadata.settlement_sources.len());
    for source in metadata.settlement_sources.iter().take(3) {
        println!("      - {}: {}", source.name, source.url);
    }

    println!("\nEvent details example finsihed!");
    Ok(())
}
