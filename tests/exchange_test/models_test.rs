use kalshi_rs::exchange::models::*;
#[test]


fn test_announcements_deserialization() {
    let json = r#"{"announcements":[]}"#;
    let _: GetExchangeAnnouncementsResponse = serde_json::from_str(json).unwrap();
}
#[test]


fn test_exchange_status_deserialization() {
    let json = r#"{"exchange_active":true,"exchange_estimated_resume_time":null,"trading_active":true}"#;
    let _: GetExcahngeStatus = serde_json::from_str(json).unwrap();
}
