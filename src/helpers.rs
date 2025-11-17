use crate::auth::Account;
use crate::auth::auth_loader::{get_current_timestamp_ms, sign_request};
use crate::errors::KalshiError;
use chrono::{DateTime, Utc};
use reqwest::{Client, StatusCode};
/// Helper functions for making authenticated and unauthenticated HTTP requests
use url::Url;
/// Create authentication headers (key_id, timestamp, signature) for a request
pub(crate) fn create_auth_headers(
    account: &Account,
    method: &str,
    path: &str,
) -> Result<(String, String, String), KalshiError> {
    // All three are required in Kalshi API headers for authenticated requests
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
    let body: String = resp.text().await?;
    if !status.is_success() {
        return Err(KalshiError::Other(format!("HTTP {}: {}", status, body)));
    }
    Ok(body)
}


/// Make an authenticated GET request
pub(crate) async fn authenticated_get<T>(
    http_client: &Client,
    base_url: &str,
    account: &Account,
    path: &str,
    json_body: Option<&T>,
) -> Result<String, KalshiError>
where
    T: serde::Serialize + ?Sized,
{
    let base = base_url.trim_end_matches('/');
    let url = format!("{}{}", base, path);
    // The signature is only computed over the path portion of the URL
    let parsed = Url::parse(&url).map_err(|e| KalshiError::Other(e.to_string()))?;
    let signed_path = parsed.path().to_string();
    let (key_id, timestamp, signature) = create_auth_headers(
        account,
        "GET",
        &signed_path,
    )?;
    // These headers are checked on every authenticated endpoint
    let mut request = http_client
        .get(parsed.as_str())
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


/// Make an authenticated POST request
pub(crate) async fn authenticated_post<T>(
    http_client: &reqwest::Client,
    base_url: &str,
    account: &Account,
    path: &str,
    json_body: Option<&T>,
) -> Result<String, KalshiError>
where
    T: serde::Serialize + ?Sized,
{
    let base = base_url.trim_end_matches('/');
    let url = format!("{}{}", base, path);
    // The signature is only computed over the path portion of the URL
    let parsed = url::Url::parse(&url).map_err(|e| KalshiError::Other(e.to_string()))?;
    let signed_path = parsed.path().to_string();
    let (key_id, timestamp, signature) = create_auth_headers(
        account,
        "POST",
        &signed_path,
    )?;
    // These headers are checked on every authenticated endpoint
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
    let text = resp.text().await?;
    if !status.is_success() {
        return Err(KalshiError::Other(format!("HTTP {}: {}", status, text)));
    }
    Ok(text)
}


///make an authenticated put request
pub(crate) async fn authenticated_put<T>(
    http_client: &reqwest::Client,
    base_url: &str,
    account: &Account,
    path: &str,
    json_body: Option<&T>,
) -> Result<(StatusCode, String), KalshiError>
where
    T: serde::Serialize + ?Sized,
{
    let base = base_url.trim_end_matches('/');
    let url = format!("{}{}", base, path);
    let parsed = url::Url::parse(&url).map_err(|e| KalshiError::Other(e.to_string()))?;
    let signed_path = parsed.path().to_string();
    let (key_id, timestamp, signature) = create_auth_headers(
        account,
        "PUT",
        &signed_path,
    )?;
    // These headers are checked on every authenticated endpoint
    let mut request = http_client
        .put(parsed.as_str())
        .header("KALSHI-ACCESS-KEY", key_id)
        .header("KALSHI-ACCESS-TIMESTAMP", &timestamp)
        .header("KALSHI-ACCESS-SIGNATURE", signature);
    if let Some(body) = json_body {
        request = request.json(body);
    }
    let resp = request.send().await?;
    let status = resp.status();
    let text = resp.text().await?;
    if !status.is_success() {
        return Err(KalshiError::Other(format!("HTTP {}: {}", status, text)));
    }
    Ok((status, text))
}


/// Make an authenticated DELETE request
pub(crate) async fn authenticated_delete<T>(
    http_client: &Client,
    base_url: &str,
    account: &Account,
    path: &str,
    body: Option<&T>,
) -> Result<(StatusCode, String), KalshiError>
where
    T: serde::Serialize + ?Sized,
{
    let base = base_url.trim_end_matches('/');
    let url = format!("{}{}", base, path);
    let parsed = Url::parse(&url).map_err(|e| KalshiError::Other(e.to_string()))?;
    let signed_path = parsed.path().to_string();
    let (key_id, timestamp, signature) = create_auth_headers(
        account,
        "DELETE",
        &signed_path,
    )?;
    // These headers are checked on every authenticated endpoint
    let mut request = http_client
        .delete(parsed.as_str())
        .header("KALSHI-ACCESS-KEY", key_id)
        .header("KALSHI-ACCESS-TIMESTAMP", &timestamp)
        .header("KALSHI-ACCESS-SIGNATURE", signature);
    if let Some(b) = body {
        request = request.json(b);
    }
    let resp = request.send().await?;
    let status = resp.status();
    let response_body = resp.text().await?;
    if !status.is_success() {
        return Err(KalshiError::Other(format!("HTTP {}: {}", status, response_body)));
    }
    Ok((status, response_body))
}


///method to convert strings to utc timestamps.. pretty useful for the responses we get back
pub(crate) fn str_to_utc(timestamp: &str) -> DateTime<Utc> {
    DateTime::parse_from_str(timestamp, "%Y-%m-%dT%H:%M:%SZ")
        .expect("Failed to parse from str to utc... string might not be in utc")
        .with_timezone(&Utc)
}
