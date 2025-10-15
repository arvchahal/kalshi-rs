/// Helper functions for making authenticated and unauthenticated HTTP requests
use url::Url;
use reqwest::Client;
use crate::auth::auth_loader::{get_current_timestamp_ms, sign_request};
use crate::errors::KalshiError;
use crate::auth::Account;

/// Create authentication headers (key_id, timestamp, signature) for a request
pub(crate) fn create_auth_headers(
    account: &Account,
    method: &str,
    path: &str,
) -> Result<(String, String, String), KalshiError> {
    let timestamp = get_current_timestamp_ms();
    let timestamp_u64: u64 = timestamp
        .parse()
        .map_err(|e| KalshiError::Other(format!("timestamp parse: {e}")))?;

    let key_id = account.key_id().trim().to_string();
    let signature = sign_request(account.private_key_pem(), method, path, timestamp_u64)
        .map_err(|e| KalshiError::Other(format!("sign error: {e}")))?;

    Ok((key_id, timestamp, signature))
}

/// Make an unauthenticated GET request (for public endpoints)
pub(crate) async fn unauthenticated_get(
    http_client: &Client,
    base_url: &str,
    path: &str,
) -> Result<String, KalshiError> {
    let base = base_url.trim_end_matches('/');
    let url = format!("{}{}", base, path);

    let resp = http_client.get(&url).send().await?;

    let status = resp.status();
    let body = resp.text().await?;

    if !status.is_success() {
        return Err(KalshiError::Other(format!("HTTP {}: {}", status, body)));
    }

    Ok(body)
}

/// Make an authenticated GET request
pub(crate) async fn authenticated_get(
    http_client: &Client,
    base_url: &str,
    account: &Account,
    path: &str,
) -> Result<String, KalshiError> {
    let base = base_url.trim_end_matches('/');
    let url = format!("{}{}", base, path);
    let parsed = Url::parse(&url).map_err(|e| KalshiError::Other(e.to_string()))?;

    let signed_path = parsed.path().to_string();
    let (key_id, timestamp, signature) = create_auth_headers(account, "GET", &signed_path)?;

    let resp = http_client
        .get(parsed.as_str())
        .header("KALSHI-ACCESS-KEY", key_id)
        .header("KALSHI-ACCESS-TIMESTAMP", &timestamp)
        .header("KALSHI-ACCESS-SIGNATURE", signature)
        .send()
        .await?;

    let status = resp.status();
    let body = resp.text().await?;

    if !status.is_success() {
        return Err(KalshiError::Other(format!("HTTP {}: {}", status, body)));
    }

    Ok(body)
}

/// Make an authenticated POST request
pub(crate) async fn authenticated_post(
    http_client: &Client,
    base_url: &str,
    account: &Account,
    path: &str,
    json_body: Option<&impl serde::Serialize>,
) -> Result<String, KalshiError> {
    let base = base_url.trim_end_matches('/');
    let url = format!("{}{}", base, path);
    let parsed = Url::parse(&url).map_err(|e| KalshiError::Other(e.to_string()))?;

    let signed_path = parsed.path().to_string();
    let (key_id, timestamp, signature) = create_auth_headers(account, "POST", &signed_path)?;

    let mut request = http_client
        .post(parsed.as_str())
        .header("KALSHI-ACCESS-KEY", key_id)
        .header("KALSHI-ACCESS-TIMESTAMP", &timestamp)
        .header("KALSHI-ACCESS-SIGNATURE", signature);

    if let Some(body) = json_body {
        request = request.json(body);
    }

    let resp = request.send().await?;
    let status = resp.status();
    let body = resp.text().await?;

    if !status.is_success() {
        return Err(KalshiError::Other(format!("HTTP {}: {}", status, body)));
    }

    Ok(body)
}

/// Make an authenticated DELETE request
pub(crate) async fn authenticated_delete(
    http_client: &Client,
    base_url: &str,
    account: &Account,
    path: &str,
) -> Result<String, KalshiError> {
    let base = base_url.trim_end_matches('/');
    let url = format!("{}{}", base, path);
    let parsed = Url::parse(&url).map_err(|e| KalshiError::Other(e.to_string()))?;

    let signed_path = parsed.path().to_string();
    let (key_id, timestamp, signature) = create_auth_headers(account, "DELETE", &signed_path)?;

    let resp = http_client
        .delete(parsed.as_str())
        .header("KALSHI-ACCESS-KEY", key_id)
        .header("KALSHI-ACCESS-TIMESTAMP", &timestamp)
        .header("KALSHI-ACCESS-SIGNATURE", signature)
        .send()
        .await?;

    let status = resp.status();
    let body = resp.text().await?;

    if !status.is_success() {
        return Err(KalshiError::Other(format!("HTTP {}: {}", status, body)));
    }

    Ok(body)
}
