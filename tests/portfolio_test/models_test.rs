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

/// A settlement exactly as `GET /portfolio/settlements` returns it today.
///
/// Kalshi replaced the integer-cent `yes_total_cost` / `no_total_cost` with
/// fixed-point `*_dollars` strings and added `event_ticker`. The old required
/// fields made every live settlements call fail to deserialize.
#[test]
fn test_settlement_deserialization_live_shape() {
    let json = r#"{"event_ticker":"KXDEPORTCOUNT-25","fee_cost":"4.776000","market_result":"no","no_count_fp":"145.00","no_total_cost_dollars":"91.174000","revenue":0,"settled_time":"2026-01-28T00:36:29.958487Z","ticker":"KXDEPORTCOUNT-25-500","value":0,"yes_count_fp":"145.00","yes_total_cost_dollars":"81.070000"}"#;
    let s: Settlement = serde_json::from_str(json).unwrap();
    assert_eq!(s.yes_total_cost_dollars, "81.070000");
    assert_eq!(s.no_total_cost_dollars, "91.174000");
    assert_eq!(s.event_ticker.as_deref(), Some("KXDEPORTCOUNT-25"));
    // Legacy integer fields are gone from the wire and must stay optional.
    assert!(s.yes_total_cost.is_none());
}

/// Balance now carries `balance_dollars` and a per-exchange breakdown; the
/// integer `balance` truncates sub-cent precision.
#[test]
fn test_balance_deserialization_live_shape() {
    let json = r#"{"balance":28,"balance_breakdown":[{"balance":"0.2800","exchange_index":0},{"balance":"0.0000","exchange_index":1}],"balance_dollars":"0.2800","portfolio_value":0,"updated_ts":1786571880}"#;
    let b: GetBalanceResponse = serde_json::from_str(json).unwrap();
    assert_eq!(b.balance, 28);
    assert_eq!(b.balance_dollars, "0.2800");
    assert_eq!(b.balance_breakdown.len(), 2);
    assert_eq!(b.balance_breakdown[1].exchange_index, 1);
}
