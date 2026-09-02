use kalshi_rs::markets::models::*;

/// A market exactly as `GET /markets` returns it today.
///
/// Deliberately omits `response_price_units` and `tick_size`: Kalshi
/// deprecated both in favour of `price_level_structure` / `price_ranges` and
/// no longer sends them. An earlier fixture still listed them, which let the
/// unit tests pass while every live call failed to deserialize.
const LIVE_MARKET: &str = r#"{
  "ticker":"KXTEST-26AUG12-T1","event_ticker":"KXTEST-26AUG12","market_type":"binary",
  "title":"Test market","yes_sub_title":"Yes","no_sub_title":"No",
  "open_time":"2026-08-12T22:06:02Z","close_time":"2026-08-16T02:10:00Z",
  "expected_expiration_time":"2026-08-13T08:10:00Z","expiration_time":"2026-08-16T02:10:00Z",
  "latest_expiration_time":"2026-08-16T02:10:00Z","settlement_timer_seconds":5,
  "status":"active","notional_value_dollars":"1.0000",
  "yes_bid_dollars":"0.4500","yes_ask_dollars":"0.4700",
  "no_bid_dollars":"0.5300","no_ask_dollars":"0.5500",
  "last_price_dollars":"0.4600","previous_yes_bid_dollars":"0.4400",
  "previous_yes_ask_dollars":"0.4600","previous_price_dollars":"0.4500",
  "volume_fp":"1200.00","volume_24h_fp":"350.00","liquidity_dollars":"820.0000",
  "open_interest_fp":"640.00","can_close_early":true,"result":"",
  "expiration_value":"","strike_type":"custom","custom_strike":{"k":"v"},
  "rules_primary":"","rules_secondary":"",
  "price_level_structure":"deci_cent",
  "price_ranges":[{"end":"1.0000","start":"0.0000","step":"0.0010"}],
  "exchange_index":1,"created_time":"2026-08-12T22:06:02.148439Z",
  "updated_time":"2026-08-12T22:06:02.214256Z","is_provisional":true,
  "yes_bid_size_fp":"25.00","yes_ask_size_fp":"30.00"
}"#;

#[test]
fn test_market_deserialization() {
    let market: Market = serde_json::from_str(LIVE_MARKET).unwrap();
    assert_eq!(market.ticker, "KXTEST-26AUG12-T1");
    assert_eq!(market.yes_bid_dollars, "0.4500");
    assert_eq!(market.price_level_structure, "deci_cent");
    assert_eq!(market.price_ranges.len(), 1);
    // New fields Kalshi added alongside the fixed-point migration.
    assert_eq!(market.exchange_index, 1);
    assert!(market.is_provisional);
    assert_eq!(market.yes_bid_size_fp.as_deref(), Some("25.00"));
    assert!(market.created_time.is_some());
    // Deprecated fields are absent from the payload and must default.
    assert_eq!(market.response_price_units, "");
    assert_eq!(market.tick_size, 0);
}

/// Payloads recorded before the migration must still deserialize.
#[test]
fn test_legacy_market_with_deprecated_fields_still_parses() {
    let legacy = LIVE_MARKET
        .trim_end_matches('}')
        .to_string()
        + r#","response_price_units":"usd_cent","tick_size":1}"#;
    let market: Market = serde_json::from_str(&legacy).unwrap();
    assert_eq!(market.response_price_units, "usd_cent");
    assert_eq!(market.tick_size, 1);
}

#[test]
fn test_get_markets_response_deserialization() {
    let json = r#"{"markets":[]}"#;
    let _: GetMarketsResponse = serde_json::from_str(json).unwrap();
}
