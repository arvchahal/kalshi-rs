use kalshi_rs::api_keys::models::*;
#[test]


fn test_api_key_deserialization() {
    let json = r#"{"api_key_id":"k","name":"n"}"#;
    let _: ApiKey = serde_json::from_str(json).unwrap();
}
#[test]


fn test_list_api_keys_response_deserialization() {
    let json = r#"{"api_keys":[]}"#;
    let _: ListApiKeysResponse = serde_json::from_str(json).unwrap();
}
