use kalshi_rs::markets::models::*;
#[test]


fn test_market_deserialization() {
    let json = r#"{"ticker":"T","event_ticker":"E","market_type":"binary","title":"","subtitle":"","yes_sub_title":"","no_sub_title":"","open_time":"","close_time":"","expected_expiration_time":"","expiration_time":"","latest_expiration_time":"","settlement_timer_seconds":0,"status":"","response_price_units":"","notional_value":0,"notional_value_dollars":"","yes_bid":0,"yes_bid_dollars":"","yes_ask":0,"yes_ask_dollars":"","no_bid":0,"no_bid_dollars":"","no_ask":0,"no_ask_dollars":"","last_price":0,"last_price_dollars":"","previous_yes_bid":0,"previous_yes_bid_dollars":"","previous_yes_ask":0,"previous_yes_ask_dollars":"","previous_price":0,"previous_price_dollars":"","volume":0,"volume_24h":0,"liquidity":0,"liquidity_dollars":"","open_interest":0,"can_close_early":false,"expiration_value":"","category":"","risk_limit_cents":0,"strike_type":"","tick_size":0,"price_level_structure":"","price_ranges":[]}"#;
    let _: Market = serde_json::from_str(json).unwrap();
}
#[test]


fn test_get_markets_response_deserialization() {
    let json = r#"{"markets":[]}"#;
    let _: GetMarketsResponse = serde_json::from_str(json).unwrap();
}
