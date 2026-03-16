use kalshi_rs::portfolio::models::*;
#[test]

fn test_order_deserialization() {
    let json = r#"{"order_id":"o","user_id":"u","client_order_id":"c","ticker":"t","side":"yes","action":"buy","type":"limit","status":"resting"}"#;
    let _: Order = serde_json::from_str(json).unwrap();
}
#[test]

fn test_fill_deserialization() {
    let json = r#"{"fill_id":"f","trade_id":"t","order_id":"o","ticker":"t","market_ticker":"m","side":"yes","action":"buy","count_fp":"1","yes_price_dollars":"0.0005","no_price_dollars":"0.0005","yes_price_fixed":"0.50","no_price_fixed":"0.50","is_taker":true,"created_time":"","ts":0}"#;
    let _: Fill = serde_json::from_str(json).unwrap();
}
