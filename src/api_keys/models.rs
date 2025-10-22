use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub api_key_id: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateApiKeyRequest {
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Display)]
#[display("CreateApiKeyResponse {{ key_id: {}, api_key: {}}}", api_key_id, private_key)]
pub struct CreateApiKeyResponse {
    pub api_key_id: String,
    pub private_key: String,
}

#[derive(Debug, Deserialize)]
pub struct ListApiKeysResponse {
    pub api_keys: Vec<ApiKey>,
}

#[derive(Debug, Deserialize, Display)]
#[display("Delete API key response {{{:?}, delete sucessful}}", &self.body)]
pub struct DeleteApiKeyResponse{
    pub body: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_key_deserialization() {
        let json = r#"{
            "api_key_id": "key_12345",
            "name": "My Trading Bot"
        }"#;

        let api_key: ApiKey = serde_json::from_str(json).unwrap();
        assert_eq!(api_key.api_key_id, "key_12345");
        assert_eq!(api_key.name, "My Trading Bot");
    }

    #[test]
    fn test_create_api_key_request_serialization() {
        let request = CreateApiKeyRequest {
            description: Some("Test key".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("Test key"));
    }

    #[test]
    fn test_create_api_key_response_deserialization() {
        let json = r#"{
            "api_key_id": "key_abc123",
            "private_key": "-----BEGIN PRIVATE KEY-----\ntest\n-----END PRIVATE KEY-----"
        }"#;

        let response: CreateApiKeyResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.api_key_id, "key_abc123");
        assert!(response.private_key.contains("BEGIN PRIVATE KEY"));
    }

    #[test]
    fn test_list_api_keys_response_deserialization() {
        let json = r#"{
            "api_keys": [
                {"api_key_id": "key1", "name": "First"},
                {"api_key_id": "key2", "name": "Second"}
            ]
        }"#;

        let response: ListApiKeysResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.api_keys.len(), 2);
        assert_eq!(response.api_keys[0].api_key_id, "key1");
        assert_eq!(response.api_keys[1].name, "Second");
    }

    #[test]
    fn test_delete_api_key_response_none() {
        let response = DeleteApiKeyResponse { body: None };
        assert!(response.body.is_none());
    }

    #[test]
    fn test_delete_api_key_response_with_body() {
        let response = DeleteApiKeyResponse {
            body: Some("Deleted".to_string()),
        };
        assert_eq!(response.body, Some("Deleted".to_string()));
    }
}