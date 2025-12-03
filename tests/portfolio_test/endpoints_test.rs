use crate::common::setup_client;
use kalshi_rs::portfolio::models::*;
use std::time::Duration;
use tokio::time::sleep;
#[tokio::test]
async fn test_get_balance() {
    let client = setup_client();
    let result = client.get_balance().await;
    assert!(result.is_ok(), "Failed to get balance: {:?}", result.err());
    let balance = result.unwrap();
    println!("Balance: {} cents", balance.balance);
    println!("Portfolio Value: {} cents", balance.portfolio_value);
}
#[tokio::test]
async fn test_get_positions_default() {
    let client = setup_client();
    let params = GetPositionsParams::default();
    let result = client.get_positions(&params).await;
    assert!(result.is_ok(), "Failed to get positions: {:?}", result.err());
    let positions = result.unwrap();
    println!("Market positions: {}", positions.market_positions.len());
    println!("Event positions: {}", positions.event_positions.len());
}
#[tokio::test]
async fn test_get_positions_with_limit() {
    let client = setup_client();
    let params = GetPositionsParams {
        limit: Some(2),
        ..Default::default()
    };
    let result = client.get_positions(&params).await;
    assert!(result.is_ok());
    let positions = result.unwrap();
    println!("{:?}", positions.market_positions.len());
    assert!(positions.market_positions.len() <= 3);
}
#[tokio::test]
async fn test_get_positions_unsettled_only() {
    let client = setup_client();
    let params = GetPositionsParams {
        settlement_status: Some("unsettled".to_string()),
        ..Default::default()
    };
    let result = client.get_positions(&params).await;
    assert!(result.is_ok());
    let positions = result.unwrap();
    println!("Unsettled positions: {}", positions.market_positions.len());
}
#[tokio::test]
async fn test_get_fills_default() {
    let client = setup_client();
    let params = GetFillsParams::default();
    let result = client.get_fills(&params).await;
    assert!(result.is_ok(), "Failed to get fills: {:?}", result.err());
    let fills = result.unwrap();
    println!("Total fills: {}", fills.fills.len());
    println!("Cursor: {}", fills.cursor);
}
#[tokio::test]
async fn test_get_fills_with_limit() {
    let client = setup_client();
    let params = GetFillsParams {
        limit: Some(5),
        ..Default::default()
    };
    let result = client.get_fills(&params).await;
    assert!(result.is_ok());
    let fills = result.unwrap();
    assert!(fills.fills.len() <= 5);
    if !fills.cursor.is_empty() {
        let next_params = GetFillsParams {
            cursor: Some(fills.cursor),
            ..Default::default()
        };
        let next_result = client.get_fills(&next_params).await;
        assert!(next_result.is_ok(), "Pagination failed");
    }
}
#[tokio::test]
async fn test_get_fills_by_ticker() {
    let client = setup_client();
    let params = GetFillsParams {
        ticker: Some("EXAMPLE-TICKER".to_string()),
        limit: Some(10),
        ..Default::default()
    };
    let result = client.get_fills(&params).await;
    if let Ok(fills) = result {
        println!("Fills for ticker: {}", fills.fills.len());
    }
}
#[tokio::test]
async fn test_get_settlements_default() {
    let client = setup_client();
    let params = GetSettlementsParams::default();
    let result = client.get_settlements(&params).await;
    assert!(result.is_ok(), "Failed to get settlements: {:?}", result.err());
    let settlements = result.unwrap();
    println!("Total settlements: {}", settlements.settlements.len());
    let losses = settlements.settlements.iter().filter(|s| s.revenue < 0).count();
    println!("Settlements with losses: {}", losses);
}
#[tokio::test]
async fn test_get_settlements_with_limit() {
    let client = setup_client();
    let params = GetSettlementsParams {
        limit: Some(10),
        ..Default::default()
    };
    let result = client.get_settlements(&params).await;
    assert!(result.is_ok());
    let settlements = result.unwrap();
    assert!(settlements.settlements.len() <= 10);
}
#[tokio::test]
async fn test_get_settlements_time_filter() {
    let client = setup_client();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let thirty_days_ago = now - (30 * 24 * 60 * 60);
    let params = GetSettlementsParams {
        min_ts: Some(thirty_days_ago),
        max_ts: Some(now),
        limit: Some(20),
        ..Default::default()
    };
    let result = client.get_settlements(&params).await;
    assert!(result.is_ok());
    let settlements = result.unwrap();
    println!("Settlements in last 30 days: {}", settlements.settlements.len());
}
#[tokio::test]
async fn test_get_orders_default() {
    let client = setup_client();
    let params = GetOrdersParams::default();
    let result = client.get_orders(&params).await;
    assert!(result.is_ok(), "Failed to get orders: {:?}", result.err());
    let orders = result.unwrap();
    println!("Total orders: {}", orders.orders.len());
}
#[tokio::test]
async fn test_get_orders_resting_only() {
    let client = setup_client();
    let params = GetOrdersParams {
        status: Some("resting".to_string()),
        limit: Some(10),
        ..Default::default()
    };
    let result = client.get_orders(&params).await;
    assert!(result.is_ok());
    let orders = result.unwrap();
    println!("Resting orders: {}", orders.orders.len());
    for order in &orders.orders {
        assert_eq!(order.status, "resting");
    }
}
#[tokio::test]
async fn test_get_single_order() {
    let client = setup_client();
    let orders = client
        .get_orders(
            &GetOrdersParams {
                limit: Some(1),
                ..Default::default()
            },
        )
        .await;
    if orders.is_err() || orders.as_ref().unwrap().orders.is_empty() {
        println!("No orders available to test get_order");
        return;
    }
    let order_id = &orders.unwrap().orders[0].order_id;
    println!("Testing with order ID: {}", order_id);
    let result = client.get_order(order_id).await;
    assert!(result.is_ok(), "Failed to get order: {:?}", result.err());
    let order = result.unwrap();
    println!("Order: {:?}", order.order);
}
#[tokio::test]
async fn test_get_order_queue_position() {
    let client = setup_client();
    let orders = client
        .get_orders(
            &GetOrdersParams {
                status: Some("resting".to_string()),
                limit: Some(1),
                ..Default::default()
            },
        )
        .await;
    if orders.is_err() || orders.as_ref().unwrap().orders.is_empty() {
        println!("No resting orders available - skipping queue position test");
        return;
    }
    let order_id = &orders.unwrap().orders[0].order_id;
    println!("Testing queue position for order ID: {}", order_id);
    let result = client.get_order_queue_position(order_id).await;
    assert!(result.is_ok(), "Failed to get queue position: {:?}", result.err());
    let queue_pos = result.unwrap();
    println!("Queue position: {}", queue_pos.queue_position);
}
#[tokio::test]
async fn test_order_group_lifecycle() {
    let client = setup_client();
    println!("1. Creating order group...");
    let create_request = CreateOrderGroupRequest {
        contracts_limit: 100,
    };
    let create_result = client.create_order_group(&create_request).await;
    assert!(create_result.is_ok(), "Failed to create: {:?}", create_result.err());
    let order_group_id = create_result.unwrap().order_group_id;
    println!("     Created: {}", order_group_id);
    sleep(Duration::from_secs(2)).await;
    println!("2. Getting order group...");
    let get_result = client.get_order_group(&order_group_id).await;
    assert!(get_result.is_ok(), "Failed to get: {:?}", get_result.err());
    let order_group = get_result.unwrap();
    println!("     Retrieved: {:?}", order_group);
    sleep(Duration::from_secs(2)).await;
    println!("3. Resetting order group... with id {}", order_group_id);
    let reset_result = client.reset_order_group(&order_group_id).await;
    assert!(reset_result.is_ok(), "Failed to reset: {:?}", reset_result.err());
    println!("     Reset successful");
    sleep(Duration::from_secs(2)).await;
    println!("4. Deleting order group...");
    let delete_result = client.delete_order_group(&order_group_id).await;
    assert!(delete_result.is_ok(), "Failed to delete: {:?}", delete_result.err());
    println!("     Deleted successfully");
}
#[tokio::test]
async fn test_get_order_groups() {
    let client = setup_client();
    let result = client.get_order_groups().await;
    assert!(result.is_ok(), "Failed to get order groups: {:?}", result.err());
    let groups = result.unwrap();
    println!("Total order groups: {}", groups.order_groups.len());
}
#[tokio::test]
async fn test_get_queue_positions() {
    let client = setup_client();
    let markets = client
        .get_all_markets(
            &kalshi_rs::markets::models::MarketsQuery {
                limit: Some(1),
                cursor: None,
                event_ticker: None,
                series_ticker: None,
                max_close_ts: None,
                min_close_ts: None,
                status: None,
                tickers: None,
            },
        )
        .await;
    if markets.is_err() || markets.as_ref().unwrap().markets.is_empty() {
        println!("Skipping - no markets available for queue position test");
        return;
    }
    let ticker = &markets.unwrap().markets[0].ticker;
    let params = GetQueueParams {
        market_tickers: Some(ticker.clone()),
        event_ticker: None,
    };
    let result = client.get_queue_positions(&params).await;
    if let Ok(positions) = result {
        println!("Queue positions: {}", positions.queue_positions.len());
    } else {
        println!("No queue positions available (this is OK if no orders in queue)");
    }
}
#[tokio::test]
async fn test_portfolio_comprehensive() {
    let client = setup_client();
    println!("\n{}", "=".repeat(80));
    println!("COMPREHENSIVE PORTFOLIO TEST");
    println!("{}\n", "=".repeat(80));
    println!("1. Getting balance...");
    let balance = client.get_balance().await.expect("Balance");
    println!(" Balance: {} cents\n", balance.balance);
    sleep(Duration::from_secs(2)).await;
    println!("2. Getting positions...");
    let positions = client
        .get_positions(&GetPositionsParams::default())
        .await
        .expect("Positions");
    println!(
        "   Positions: {} markets, {} events\n", positions.market_positions.len(),
        positions.event_positions.len()
    );
    sleep(Duration::from_secs(2)).await;
    println!("3. Getting fills...");
    let fills = client
        .get_fills(
            &GetFillsParams {
                limit: Some(5),
                ..Default::default()
            },
        )
        .await
        .expect("Fills");
    println!("   Recent fills: {}\n", fills.fills.len());
    sleep(Duration::from_secs(2)).await;
    println!("4. Getting settlements...");
    let settlements = client
        .get_settlements(
            &GetSettlementsParams {
                limit: Some(5),
                ..Default::default()
            },
        )
        .await
        .expect("Settlements");
    println!("  Recent settlements: {}\n", settlements.settlements.len());
    sleep(Duration::from_secs(2)).await;
    println!("5. Getting orders...");
    let orders = client
        .get_orders(
            &GetOrdersParams {
                limit: Some(5),
                ..Default::default()
            },
        )
        .await
        .expect("Orders");
    println!("  Recent orders: {}\n", orders.orders.len());
    sleep(Duration::from_secs(2)).await;
    println!("6. Testing order group lifecycle...");
    let og = CreateOrderGroupRequest {
        contracts_limit: 10,
    };
    let created = client.create_order_group(&og).await.expect("Create order group");
    sleep(Duration::from_secs(2)).await;
    let _ = client
        .get_order_group(&created.order_group_id)
        .await
        .expect("Get order group");
    sleep(Duration::from_secs(2)).await;
    let _ = client
        .reset_order_group(&created.order_group_id)
        .await
        .expect("Reset order group");
    sleep(Duration::from_secs(2)).await;
    let _ = client
        .delete_order_group(&created.order_group_id)
        .await
        .expect("Delete order group");
    println!("  Order group lifecycle complete\n");
    println!("{}", "=".repeat(80));
    println!("ALL PORTFOLIO TESTS PASSED");
    println!("{}\n", "=".repeat(80));
}
