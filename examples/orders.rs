// Create, cancel, and batch manage orders
//
// Run with: cargo run --example orders

use kalshi_rust_sdk::KalshiClient;
use kalshi_rust_sdk::auth::Account;
use kalshi_rust_sdk::markets::models::*;
use kalshi_rust_sdk::portfolio::models::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key_id = std::env::var("KALSHI_API_KEY_ID")
        .expect("KALSHI_API_KEY_ID must be set");
    let account = Account::from_file("kalshi_private.pem", api_key_id)?;
    let client = KalshiClient::new(account);

    // get an active market to trade
    println!("Finding an active market...");
    let params = MarketsQuery {
        limit: Some(1),
        cursor: None,
        event_ticker: None,
        series_ticker: None,
        max_close_ts: None,
        min_close_ts: None,
        status: Some("active".to_string()),
        tickers: None,
    };
    let markets = client.get_all_markets(&params).await?;

    if markets.markets.is_empty() {
        println!("No active markets found");
        return Ok(());
    }

    let ticker = &markets.markets[0].ticker;
    println!("Trading on market: {}\n", ticker);

    // single order create + cancel
    println!("1. Creating single order...");
    let order_request = CreateOrderRequest {
        ticker: ticker.clone(),
        action: "buy".to_string(),
        side: "yes".to_string(),
        count: 1,
        client_order_id: None,
        type_: Some("limit".to_string()),
        yes_price: Some(1), // 1 cent - very low price to avoid fill
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

    let created = client.create_order(&order_request).await?;
    let order_id = created.order.order_id.clone();
    println!("   Created order: {}", order_id);
    println!("   Status: {}", created.order.status);
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // cancel the order
    println!("\n2. Canceling order...");
    let canceled = client.cancel_order(order_id.clone()).await?;
    println!("   Canceled order: {}", order_id);
    println!("   Final status: {}\n", canceled.order.status);


    // The following is only available to users with advanced API access as such these tests might fail if you run them
    // only use if you have advanced api access likely will know if you have it!


    // batch orders: create multiple and cancel all




    // println!("3. Creating batch of orders...");

    // // create 3 orders
    // let mut order_ids = Vec::new();
    // for i in 1..=2 {
    //     let req = CreateOrderRequest {
    //         ticker: ticker.clone(),
    //         action: "buy".to_string(),
    //         side: "yes".to_string(),
    //         count: 1,
    //         client_order_id: None,
    //         type_: Some("limit".to_string()),
    //         yes_price: Some(1), // 1 cent
    //         no_price: None,
    //         yes_price_dollars: None,
    //         no_price_dollars: None,
    //         expiration_ts: None,
    //         time_in_force: None,
    //         buy_max_cost: None,
    //         post_only: None,
    //         reduce_only: None,
    //         self_trade_prevention_type: None,
    //         order_group_id: None,
    //         cancel_order_on_pause: None,
    //     };

    //     let created = client.create_order(&req).await?;
    //     order_ids.push(created.order.order_id.clone());
    //     println!("   Created: {}", created.order.order_id);
    //     println!("Created order {} -> status {}", created.order.order_id, created.order.status);

    // }

    // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // // batch cancel all orders
    // println!("\n4. Batch canceling all orders...");
    // let batch_cancel_req: BatchCancelOrdersRequest = BatchCancelOrdersRequest {
    //     order_ids: order_ids,
    // };

    // let canceled_orders = client.batch_cancel_orders(&batch_cancel_req).await?;
    // println!("Canceled {} orders:", canceled_orders.orders.len());
    // for order in &canceled_orders.orders {
    //     println!("- {} (status: {})", order.order_id, order.status);
    // }

    // println!("\nOrder management example completed!");
    Ok(())
}
