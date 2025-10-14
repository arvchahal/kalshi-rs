use std::error::Error;
use url::Url;
use crate::client::KalshiClient;
use crate::api_keys::models::{ApiKey, CreateApiKeyRequest, CreateApiKeyResponse, ListApiKeysResponse};
use crate::errors::KalshiError;
use crate::auth::auth_loader::{get_current_timestamp_ms,sign_request};

// Endpoints for creating/deleting/generating/getting api keys
// All of these methods require auth including your private key,
// your signed text with your key, and the timestamp

/*
From Kalshi docs:
KALSHI-ACCESS-KEY - the Key ID
KALSHI-ACCESS-TIMESTAMP - the request timestamp in ms
KALSHI-ACCESS-SIGNATURE - request hash signed with private key
*/

const GET_API_KEY: &str = "/trade-api/v2/api_keys/"; // get
const CREATE_API_KEY: &str = "/trade-api/v2/api_keys/"; // post
const DELETE_API_KEY: &str = "/trade-api/v2/api_keys/generate/"; // post
const GENERATE_API_KEY: &str = "/trade-api/v2/api_keys/{api_key}/"; // delete

fn full_url(prefix: &str, suffix: &str) -> String {
   let x = format!("{}{}", prefix, suffix); // e.g. "/trade-api/v2/api_keys"
    println!("{}",x);
    x
}




impl KalshiClient {
    /// GET /api-keys
    /// List all API keys for the authenticated user
    pub async fn get_api_keys(&self) -> Result<Vec<ApiKey>, KalshiError> {
    // base_url should be host only, e.g. "https://api.elections.kalshi.com"
    let base = self.base_url.trim_end_matches('/');
    let url = format!("{}{}", base, GET_API_KEY);
    let parsed = Url::parse(&url).map_err(|e| KalshiError::Other(e.to_string()))?;

    // SIGN EXACTLY the path we will request
    let signed_path = parsed.path().to_string(); // e.g. "/trade-api/v2/api_keys"

    let ts = get_current_timestamp_ms();                 // ms as string
    let ts_u64: u64 = ts.parse().map_err(|e| KalshiError::Other(format!("timestamp parse: {e}")))?;

    let key_id = self.account.key_id().trim();           // trim in case of hidden whitespace
    let sig = sign_request(&self.account.private_key_pem(), "GET", &signed_path, ts_u64)
        .map_err(|e| KalshiError::Other(format!("sign error: {e}")))?;

    // Optional debug
    eprintln!("URL  : {}", url);
    eprintln!("SIGNED: {}{}{}", ts, "GET", signed_path);
    eprintln!("HEADERS:");
    eprintln!("  KALSHI-ACCESS-KEY: '{}'", key_id);
    eprintln!("  KALSHI-ACCESS-KEY (len): {}", key_id.len());
    eprintln!("  KALSHI-ACCESS-TIMESTAMP: '{}'", ts);
    eprintln!("  KALSHI-ACCESS-SIGNATURE: '{}'", sig);

    let resp = self.http_client
        .get(parsed.as_str())
        .header("KALSHI-ACCESS-KEY", key_id)
        .header("KALSHI-ACCESS-TIMESTAMP", &ts)
        .header("KALSHI-ACCESS-SIGNATURE", sig)
        .send()
        .await?;

    let status = resp.status();
    let body = resp.text().await?;
    if !status.is_success() {
        return Err(KalshiError::Other(format!("HTTP {}: {}", status, body)));
    }

    let data: ListApiKeysResponse = serde_json::from_str(&body)
        .map_err(|e| KalshiError::Other(format!("Parse error: {e}. Body: {body}")))?;
    Ok(data.api_keys)
}

    /// POST /users/api-keys/generate
    /// Generate a new API key (alternative endpoint)
    pub async fn generate_api_key(&self) -> Result<CreateApiKeyResponse, KalshiError> {
        let url = format!("{}/{}", self.base_url, DELETE_API_KEY);

        let timestamp_str = get_current_timestamp_ms();
        let timestamp: u64 = timestamp_str.parse()
            .map_err(|e| KalshiError::Other(format!("Failed to parse timestamp: {}", e)))?;

        let key_id = self.account.key_id();
        let signature = sign_request(&self.account.private_key_pem(), "POST", DELETE_API_KEY, timestamp)
            .map_err(|e| KalshiError::Other(format!("Failed to sign request: {}", e)))?;

        let response = self.http_client
            .post(&url)
            .header("KALSHI-ACCESS-KEY", key_id)
            .header("KALSHI-ACCESS-TIMESTAMP", timestamp_str)
            .header("KALSHI-ACCESS-SIGNATURE", signature)
            .send()
            .await?;

        Ok(response.json().await?)
    }
}
