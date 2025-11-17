// Get account balance, positions, fills, and orders
//
// Run with: cargo run --example get_balances

use kalshi_rust_sdk::KalshiClient;
use kalshi_rust_sdk::auth::Account;
use kalshi_rust_sdk::portfolio::models::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup authentication
    let api_key_id = std::env::var("KALSHI_API_KEY_ID")
        .expect("KALSHI_API_KEY_ID must be set");
    let account = Account::from_file("kalshi_private.pem", api_key_id)?;
    let client = KalshiClient::new(account);

    // Get balance
    let balance = client.get_balance().await?;
    println!("Account Balance:");
    println!("  Available: ${:.2}", balance.balance as f64 / 100.0);
    println!("  Portfolio Value: ${:.2}", balance.portfolio_value as f64 / 100.0);

    // Get positions to see what's locked up
    let positions_params = GetPositionsParams {
        limit: Some(10),
        ..Default::default()
    };
    let positions = client.get_positions(&positions_params).await?;
    println!("\nOpen Positions ({}):", positions.market_positions.len());
    for pos in positions.market_positions.iter().take(5) {
        if let (Some(ticker), Some(position)) = (&pos.market_ticker, pos.position) {
            println!("  {} - {} contracts", ticker, position);
        }
    }

    // Get recent fills to see trading activity
    let fills_params = GetFillsParams {
        limit: Some(5),
        ..Default::default()
    };
    let fills = client.get_fills(&fills_params).await?;
    println!("\nRecent Fills ({}):", fills.fills.len());
    for fill in fills.fills.iter() {
        println!("  {} - {} @ ${:.2}", fill.ticker, fill.count, fill.price as f64 / 100.0);
    }

    // Get pending orders
    let orders_params = GetOrdersParams {
        status: Some("resting".to_string()),
        limit: Some(10),
        ..Default::default()
    };
    let orders = client.get_orders(&orders_params).await?;
    println!("\nPending Orders ({}):", orders.orders.len());
    for order in orders.orders.iter().take(5) {
        if let (Some(remaining), Some(price)) = (order.remaining_count, order.yes_price) {
            println!("  {} - {} @ {} cents", order.ticker, remaining, price);
        }
    }

    Ok(())
}