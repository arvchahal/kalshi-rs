use kalshi_rust_sdk::KalshiClient;
use kalshi_rust_sdk::auth::Account;
use kalshi_rust_sdk::portfolio::models::*;
use kalshi_rust_sdk::markets::models::MarketsQuery;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup authentication
    let api_key_id = std::env::var("KALSHI_API_KEY_ID")
        .expect("KALSHI_API_KEY_ID environment variable must be set");
    let account = Account::from_file("kalshi_private.pem", api_key_id)?;
    let client = KalshiClient::new(account);

    // Search for active markets directly
    println!("Searching for active markets...");
    let params = MarketsQuery {
        limit: Some(10),
        cursor: None,
        event_ticker: None,
        series_ticker: None,
        max_close_ts: None,
        min_close_ts: None,
        status: Some("active".to_string()),
        tickers: None,
    };
    let markets = client.get_all_markets(&params).await?;

    if !markets.markets.is_empty() {
        let first_market = &markets.markets[0];
        println!("\nFirst active market:");
        println!("  Ticker: {}", first_market.ticker);
        println!("  Title: {}", first_market.title);
        println!("  Subtitle: {}", first_market.subtitle);
        println!("  Status: {}", first_market.status);

        println!("\n{}", "=".repeat(80));
        println!("Testing order create and cancel with market: {}", first_market.ticker);
        match test_create_and_cancel_order(&client, &first_market.ticker).await {
            Ok(_) => println!("\norder test ok"),
            Err(e) => {
                println!("\nORDER FAILED: {e:?}");
                return Err(e);
            }
        }
    } else {
        println!("No active markets found, skipping order test");
    }

    Ok(())
}

// =============================================================================
// TRADING TESTS (COSTS MONEY - BE CAREFUL!)
// =============================================================================

async fn test_create_and_cancel_order(client: &KalshiClient, ticker: &str) -> Result<(), Box<dyn std::error::Error>> {

    let create_req = CreateOrderRequest {
        ticker: ticker.to_string(),
        action: "buy".to_string(),
        side: "yes".to_string(),
        count: 1,
        client_order_id: None,
        type_: Some("limit".to_string()),
        yes_price: Some(20), // 1 cent - lowest possible price to minimize cost
        no_price: None,
        yes_price_dollars: None,
        no_price_dollars: None,
        expiration_ts: None,
        time_in_force: None,
        buy_max_cost: None,
        post_only: None,
        reduce_only: None,
        self_trade_prevention_type: None,
        order_group_id: None,
        cancel_order_on_pause: None,
    };

    println!("Creating order: {} {} @ {} cents...", create_req.action, ticker, 1);
    let created = client.create_order(&create_req).await?;
    println!("hello");
    print!("{}",created);
    let order_id = created.order.order_id.clone();
    println!("   Created order: {}", order_id);

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Cancel immediately
    // println!("   Canceling order...");
    // let canceled = client.cancel_order(order_id.clone()).await?;
    // println!("   Canceled order: {:?}", canceled.order);
    // println!();

    Ok(())
}

// // async fn test_batch_create_and_cancel(client: &KalshiClient) -> Result<(), Box<dyn std::error::Error>> {
// //     println!("Testing: Batch Create and Cancel Orders");
// //     println!("WARNING: THIS WILL CREATE MULTIPLE REAL ORDERS");

// //     // NOTE: Batch create uses Order objects, not CreateOrderRequest
// //     // You need to manually create orders first, then pass them to batch
// //     println!("   Skipping - batch operations require existing order IDs");
// //     println!();

// //     Ok(())
// // }

// // async fn test_amend_order(client: &KalshiClient) -> Result<(), Box<dyn std::error::Error>> {
// //     println!("Testing: Amend Order");
// //     println!("WARNING: THIS WILL CREATE AND AMEND A REAL ORDER");

// //     let ticker = "YOUR-MARKET-TICKER-HERE";

// //     // Create order
// //     let create_req = CreateOrderRequest {
// //         ticker: ticker.to_string(),
// //         action: "buy".to_string(),
// //         side: "yes".to_string(),
// //         count: 2,
// //         client_order_id: Some("test-client-id".to_string()),
// //         type_: Some("limit".to_string()),
// //         yes_price: Some(1),
// //         no_price: None,
// //         yes_price_dollars: None,
// //         no_price_dollars: None,
// //         expiration_ts: None,
// //         time_in_force: None,
// //         buy_max_cost: None,
// //         post_only: None,
// //         reduce_only: None,
// //         self_trade_prevention_type: None,
// //         order_group_id: None,
// //         cancel_order_on_pause: None,
// //     };

// //     println!("   Creating order...");
// //     let created = client.create_order(&create_req).await?;
// //     let order_id = created.order.order_id.clone();
// //     println!("   Created: {}", order_id);

// //     tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

// //     // Amend to different price
// //     let amend_req = AmendOrderRequest {
// //         ticker: ticker.to_string(),
// //         side: "yes".to_string(),
// //         action: "buy".to_string(),
// //         client_order_id: "test-client-id".to_string(),
// //         updated_client_order_id: "test-client-id-updated".to_string(),
// //         yes_price: Some(2), // Change from 1 cent to 2 cents
// //         no_price: None,
// //         yes_price_dollars: None,
// //         no_price_dollars: None,
// //         count: Some(2),
// //     };

// //     println!("   Amending order to 2 cents...");
// //     let amended = client.amend_order(&order_id, &amend_req).await?;
// //     println!("   Amended: {:?}", amended.order);

// //     tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

// //     // Cancel
// //     println!("   Canceling...");
// //     client.cancel_order(order_id).await?;
// //     println!("   Canceled");
// //     println!();

// //     Ok(())
// // }

// // async fn test_decrease_order(client: &KalshiClient) -> Result<(), Box<dyn std::error::Error>> {
// //     println!("Testing: Decrease Order");
// //     println!("WARNING: THIS WILL CREATE A REAL ORDER");

// //     let ticker = "YOUR-MARKET-TICKER-HERE";

// //     // Create order with count=3
// //     let create_req = CreateOrderRequest {
// //         ticker: ticker.to_string(),
// //         action: "buy".to_string(),
// //         side: "yes".to_string(),
// //         count: 3,
// //         client_order_id: None,
// //         type_: Some("limit".to_string()),
// //         yes_price: Some(1),
// //         no_price: None,
// //         yes_price_dollars: None,
// //         no_price_dollars: None,
// //         expiration_ts: None,
// //         time_in_force: None,
// //         buy_max_cost: None,
// //         post_only: None,
// //         reduce_only: None,
// //         self_trade_prevention_type: None,
// //         order_group_id: None,
// //         cancel_order_on_pause: None,
// //     };

// //     println!("   Creating order with count=3...");
// //     let created = client.create_order(&create_req).await?;
// //     let order_id = created.order.order_id.clone();
// //     println!("   Created: {}", order_id);

// //     tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

// //     // Decrease to count=1
// //     let decrease_req = DecreaseOrderRequest {
// //         reduce_by: 2, // Reduce by 2, leaving 1
// //         reduce_to: 1, // Target count of 1
// //     };

// //     println!("   Decreasing order by 2...");
// //     let decreased = client.decrease_order(&order_id, &decrease_req).await?;
// //     println!("   Decreased: {:?}", decreased.order);

// //     tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

// //     // Cancel remaining
// //     println!("   Canceling...");
// //     client.cancel_order(order_id).await?;
// //     println!("   Canceled");
// //     println!();

// //     Ok(())
// }
