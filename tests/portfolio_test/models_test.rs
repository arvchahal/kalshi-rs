use kalshi_rust_sdk::portfolio::models::*;
use serde_json;
#[test]
fn test_order_deserialization() {
    let json = r#"{
        "order_id": "order123",
        "user_id": "user456",
        "client_order_id": "client789",
        "ticker": "MARKET-123",
        "side": "yes",
        "action": "buy",
        "type": "limit",
        "status": "resting",
        "yes_price": 50,
        "no_price": 50,
        "yes_price_dollars": "0.50",
        "no_price_dollars": "0.50",
        "fill_count": 10,
        "remaining_count": 90,
        "initial_count": 100,
        "taker_fees": 5,
        "maker_fees": 0,
        "taker_fill_cost": 500,
        "maker_fill_cost": 0,
        "taker_fill_cost_dollars": "5.00",
        "maker_fill_cost_dollars": "0.00",
        "queue_position": 5,
        "taker_fees_dollars": "0.05",
        "created_time": "2023-11-07T05:31:56Z",
        "last_update_time": "2023-11-07T05:31:56Z"
    }"#;
    let order: Order = serde_json::from_str(json).unwrap();
    assert_eq!(order.order_id, "order123");
    assert_eq!(order.ticker, "MARKET-123");
    assert_eq!(order.side, "yes");
    assert_eq!(order.action, "buy");
    assert_eq!(order.type_, "limit");
    assert_eq!(order.status, "resting");
    assert_eq!(order.yes_price, Some(50));
    assert_eq!(order.fill_count, Some(10));
    assert_eq!(order.remaining_count, Some(90));
}
#[test]
fn test_order_with_error_deserialization() {
    let json = r#"{
        "order_id": "order123",
        "user_id": "user456",
        "client_order_id": "",
        "ticker": "MARKET-123",
        "side": "yes",
        "action": "buy",
        "type": "limit",
        "status": "error",
        "order_error": {
            "code": "insufficient_balance",
            "message": "Insufficient balance",
            "details": "Need $100, have $50"
        }
    }"#;
    let order: Order = serde_json::from_str(json).unwrap();
    assert_eq!(order.status, "error");
    assert!(order.order_error.is_some());
    let error = order.order_error.unwrap();
    assert_eq!(error.code, Some("insufficient_balance".to_string()));
    assert_eq!(error.message, Some("Insufficient balance".to_string()));
}
#[test]
fn test_create_order_request_serialization() {
    let request = CreateOrderRequest {
        ticker: "MARKET-123".to_string(),
        side: "yes".to_string(),
        action: "buy".to_string(),
        count: 100,
        client_order_id: Some("my-order-1".to_string()),
        type_: Some("limit".to_string()),
        yes_price: Some(50),
        no_price: None,
        yes_price_dollars: None,
        no_price_dollars: None,
        expiration_ts: None,
        time_in_force: None,
        buy_max_cost: None,
        post_only: Some(true),
        reduce_only: None,
        self_trade_prevention_type: None,
        order_group_id: None,
        cancel_order_on_pause: None,
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("MARKET-123"));
    assert!(json.contains("\"count\":100"));
    assert!(json.contains("\"post_only\":true"));
}
#[test]
fn test_amend_order_request_serialization() {
    let request = AmendOrderRequest {
        ticker: "MARKET-123".to_string(),
        side: "yes".to_string(),
        action: "buy".to_string(),
        client_order_id: "old-id".to_string(),
        updated_client_order_id: "new-id".to_string(),
        yes_price: Some(60),
        no_price: None,
        yes_price_dollars: None,
        no_price_dollars: None,
        count: Some(150),
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("\"yes_price\":60"));
    assert!(json.contains("\"count\":150"));
}
#[test]
fn test_get_balance_response_deserialization() {
    let json = r#"{
        "balance": 10000,
        "portfolio_value": 12500,
        "updated_ts": 1699344716000
    }"#;
    let balance: GetBalanceResponse = serde_json::from_str(json).unwrap();
    assert_eq!(balance.balance, 10000);
    assert_eq!(balance.portfolio_value, 12500);
    assert_eq!(balance.updated_ts, 1699344716000);
}
#[test]
fn test_get_balance_response_zero_values() {
    let json = r#"{
        "balance": 0,
        "portfolio_value": 0,
        "updated_ts": 1699344716000
    }"#;
    let balance: GetBalanceResponse = serde_json::from_str(json).unwrap();
    assert_eq!(balance.balance, 0);
    assert_eq!(balance.portfolio_value, 0);
}
#[test]
fn test_market_position_deserialization() {
    let json = r#"{
        "market_ticker": "MARKET-123",
        "position": -100,
        "market_exposure": -5000,
        "realized_pnl": -250,
        "fees_paid": 10,
        "resting_order_count": 2,
        "total_traded": 150
    }"#;
    let position: MarketPosition = serde_json::from_str(json).unwrap();
    assert_eq!(position.market_ticker, Some("MARKET-123".to_string()));
    assert_eq!(position.position, Some(- 100));
    assert_eq!(position.market_exposure, Some(- 5000));
    assert_eq!(position.realized_pnl, Some(- 250));
    assert_eq!(position.fees_paid, Some(10));
}
#[test]
fn test_event_position_deserialization() {
    let json = r#"{
        "event_ticker": "EVENT-456",
        "position": 200,
        "event_exposure": 10000,
        "realized_pnl": 500,
        "fees_paid": 25,
        "resting_order_count": 1,
        "total_traded": 300
    }"#;
    let position: EventPosition = serde_json::from_str(json).unwrap();
    assert_eq!(position.event_ticker, Some("EVENT-456".to_string()));
    assert_eq!(position.position, Some(200));
    assert_eq!(position.event_exposure, Some(10000));
    assert_eq!(position.realized_pnl, Some(500));
}
#[test]
fn test_get_positions_params_default() {
    let params = GetPositionsParams::default();
    assert!(params.cursor.is_none());
    assert!(params.limit.is_none());
    assert!(params.count_filter.is_none());
    assert!(params.settlement_status.is_none());
    assert!(params.ticker.is_none());
    assert!(params.event_ticker.is_none());
}
#[test]
fn test_get_positions_params_serialization() {
    let params = GetPositionsParams {
        limit: Some(50),
        settlement_status: Some("unsettled".to_string()),
        ticker: Some("MARKET-123".to_string()),
        ..Default::default()
    };
    let query = serde_urlencoded::to_string(&params).unwrap();
    assert!(query.contains("limit=50"));
    assert!(query.contains("settlement_status=unsettled"));
    assert!(query.contains("ticker=MARKET-123"));
    assert!(! query.contains("cursor"));
    assert!(! query.contains("count_filter"));
}
#[test]
fn test_fill_deserialization() {
    let json = r#"{
        "fill_id": "fill123",
        "trade_id": "trade456",
        "order_id": "order789",
        "client_order_id": "client1",
        "ticker": "MARKET-123",
        "market_ticker": "MARKET-123",
        "side": "yes",
        "action": "buy",
        "count": 10,
        "price": 0.55,
        "yes_price": 0.55,
        "no_price": 0.45,
        "yes_price_fixed": "0.5500",
        "no_price_fixed": "0.4500",
        "is_taker": true,
        "created_time": "2023-11-07T05:31:56Z",
        "ts": 1699344716000
    }"#;
    let fill: Fill = serde_json::from_str(json).unwrap();
    assert_eq!(fill.fill_id, "fill123");
    assert_eq!(fill.count, 10);
    assert_eq!(fill.price, 0.55);
    assert_eq!(fill.yes_price, 0.55);
    assert_eq!(fill.no_price, 0.45);
    assert!(fill.is_taker);
}
#[test]
fn test_fill_with_optional_client_order_id() {
    let json = r#"{
        "fill_id": "fill123",
        "trade_id": "trade456",
        "order_id": "order789",
        "ticker": "MARKET-123",
        "market_ticker": "MARKET-123",
        "side": "yes",
        "action": "buy",
        "count": 10,
        "price": 0.55,
        "yes_price": 0.55,
        "no_price": 0.45,
        "yes_price_fixed": "0.5500",
        "no_price_fixed": "0.4500",
        "is_taker": false,
        "created_time": "2023-11-07T05:31:56Z",
        "ts": 1699344716000
    }"#;
    let fill: Fill = serde_json::from_str(json).unwrap();
    assert_eq!(fill.client_order_id, None);
    assert!(! fill.is_taker);
}
#[test]
fn test_get_fills_params_serialization() {
    let params = GetFillsParams {
        ticker: Some("MARKET-123".to_string()),
        limit: Some(10),
        min_ts: Some(1699344716),
        ..Default::default()
    };
    let query = serde_urlencoded::to_string(&params).unwrap();
    assert!(query.contains("ticker=MARKET-123"));
    assert!(query.contains("limit=10"));
    assert!(query.contains("min_ts=1699344716"));
}
#[test]
fn test_settlement_with_profit() {
    let json = r#"{
        "ticker": "MARKET-123",
        "market_result": "yes",
        "yes_count": 100,
        "yes_total_cost": 5000,
        "no_count": 0,
        "no_total_cost": 0,
        "revenue": 4966,
        "settled_time": "2023-11-07T05:31:56Z",
        "fee_cost": "34.00",
        "value": 10000
    }"#;
    let settlement: Settlement = serde_json::from_str(json).unwrap();
    assert_eq!(settlement.ticker, "MARKET-123");
    assert_eq!(settlement.market_result, "yes");
    assert_eq!(settlement.revenue, 4966);
    assert_eq!(settlement.fee_cost, Some("34.00".to_string()));
}
#[test]
fn test_settlement_with_loss() {
    let json = r#"{
        "ticker": "MARKET-123",
        "market_result": "no",
        "yes_count": 100,
        "yes_total_cost": 5000,
        "no_count": 0,
        "no_total_cost": 0,
        "revenue": -5034,
        "settled_time": "2023-11-07T05:31:56Z",
        "value": 0
    }"#;
    let settlement: Settlement = serde_json::from_str(json).unwrap();
    assert_eq!(settlement.market_result, "no");
    assert_eq!(settlement.revenue, - 5034);
    assert_eq!(settlement.fee_cost, None);
}
#[test]
fn test_get_settlements_params_time_range() {
    let params = GetSettlementsParams {
        min_ts: Some(1699344716),
        max_ts: Some(1699431116),
        limit: Some(20),
        ..Default::default()
    };
    let query = serde_urlencoded::to_string(&params).unwrap();
    assert!(query.contains("min_ts=1699344716"));
    assert!(query.contains("max_ts=1699431116"));
    assert!(query.contains("limit=20"));
}
#[test]
fn test_create_order_group_request_serialization() {
    let request = CreateOrderGroupRequest {
        contracts_limit: 100,
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("\"contracts_limit\":100"));
}
#[test]
fn test_create_order_group_response_deserialization() {
    let json = r#"{
        "order_group_id": "og-12345"
    }"#;
    let response: CreateOrderGroupResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.order_group_id, "og-12345");
}
#[test]
fn test_get_order_group_response() {
    let json = r#"{
        "is_auto_cancel_enabled": true,
        "orders": ["order1", "order2", "order3"]
    }"#;
    let response: GetOrderGroupResponse = serde_json::from_str(json).unwrap();
    assert!(response.is_auto_cancel_enabled);
    assert_eq!(response.orders.len(), 3);
    assert_eq!(response.orders[0], "order1");
}
#[test]
fn test_order_group_deserialization() {
    let json = r#"{
        "id": "og-123",
        "is_auto_cancel_enabled": false
    }"#;
    let order_group: OrderGroup = serde_json::from_str(json).unwrap();
    assert_eq!(order_group.id, "og-123");
    assert!(! order_group.is_auto_cancel_enabled);
}
#[test]
fn test_batch_cancel_orders_request_serialization() {
    let request = BatchCancelOrdersRequest {
        order_ids: vec!["order1".to_string(), "order2".to_string()],
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("order1"));
    assert!(json.contains("order2"));
}
#[test]
fn test_batch_cancel_orders_response_deserialization() {
    let json = r#"{
        "orders": []
    }"#;
    let response: BatchCancelOrdersResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.orders.len(), 0);
}
#[test]
fn test_decrease_order_request_serialization() {
    let request = DecreaseOrderRequest {
        reduce_by: 10,
        reduce_to: 90,
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("\"reduce_by\":10"));
    assert!(json.contains("\"reduce_to\":90"));
}
#[test]
fn test_get_order_queue_position_response() {
    let json = r#"{
        "queue_position": 5
    }"#;
    let response: GetOrderQueuePositionResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.queue_position, 5);
}
#[test]
fn test_queue_position_obj_deserialization() {
    let json = r#"{
        "order_id": "order123",
        "market_ticker": "MARKET-123",
        "queue_position": 10
    }"#;
    let obj: QueuePositionObj = serde_json::from_str(json).unwrap();
}
#[test]
fn test_get_queue_params_serialization() {
    let params = GetQueueParams {
        market_tickers: Some("MARKET-1,MARKET-2".to_string()),
        event_ticker: None,
    };
    let query = serde_urlencoded::to_string(&params).unwrap();
    assert!(query.contains("market_tickers=MARKET-1%2CMARKET-2"));
}
#[test]
fn test_get_total_resting_order_value_response() {
    let json = r#"{
        "total_resting_order_value": 5000
    }"#;
    let response: GetTotalRestingOrderValueResponse = serde_json::from_str(json)
        .unwrap();
    assert_eq!(response.total_resting_order_value, 5000);
}
#[test]
fn test_get_orders_params_all_filters() {
    let params = GetOrdersParams {
        ticker: Some("MARKET-123".to_string()),
        event_ticker: Some("EVENT-456".to_string()),
        min_ts: Some(1699344716),
        max_ts: Some(1699431116),
        status: Some("resting".to_string()),
        limit: Some(100),
        cursor: Some("cursor123".to_string()),
    };
    let query = serde_urlencoded::to_string(&params).unwrap();
    assert!(query.contains("ticker=MARKET-123"));
    assert!(query.contains("event_ticker=EVENT-456"));
    assert!(query.contains("status=resting"));
    assert!(query.contains("limit=100"));
    assert!(query.contains("cursor=cursor123"));
}
#[test]
fn test_order_with_all_none_optionals() {
    let json = r#"{
        "order_id": "order123",
        "user_id": "user456",
        "client_order_id": "",
        "ticker": "MARKET-123",
        "side": "yes",
        "action": "buy",
        "type": "limit",
        "status": "resting"
    }"#;
    let order: Order = serde_json::from_str(json).unwrap();
    assert_eq!(order.order_id, "order123");
    assert!(order.yes_price.is_none());
    assert!(order.fill_count.is_none());
    assert!(order.order_error.is_none());
}
#[test]
fn test_positions_response_empty_vectors() {
    let json = r#"{
        "cursor": null,
        "market_positions": [],
        "event_positions": []
    }"#;
    let response: GetPositionsResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.market_positions.len(), 0);
    assert_eq!(response.event_positions.len(), 0);
    assert!(response.cursor.is_none());
}
#[test]
fn test_fills_response_with_cursor() {
    let json = r#"{
        "fills": [],
        "cursor": "next_page_token"
    }"#;
    let response: GetFillsResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.fills.len(), 0);
    assert_eq!(response.cursor, "next_page_token");
}
#[test]
fn test_settlements_response_with_cursor() {
    let json = r#"{
        "settlements": [],
        "cursor": "settlement_cursor"
    }"#;
    let response: GetSettlementsResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.settlements.len(), 0);
    assert_eq!(response.cursor, "settlement_cursor");
}
