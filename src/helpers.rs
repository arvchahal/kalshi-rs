use crate::auth::auth_loader::{get_current_timestamp_ms, sign_request};
use crate::auth::Account;
use crate::errors::KalshiError;
use crate::rate_limit::{RateLimiter, RequestKind, DEFAULT_REQUEST_COST};
use chrono::{DateTime, Utc};
use reqwest::{Client, Method, StatusCode};
/// Helper functions for making authenticated and unauthenticated HTTP requests
use url::Url;

/// Send a request, pacing it through `limiter` and retrying on `HTTP 429`.
///
/// Each attempt is signed afresh because Kalshi's signature covers the
/// timestamp, which must stay close to the server's clock. Kalshi does not
/// send `Retry-After`, so retries use the limiter's exponential backoff.
pub(crate) async fn send_request<T>(
    http_client: &Client,
    base_url: &str,
    account: Option<&Account>,
    limiter: Option<&RateLimiter>,
    method: Method,
    path: &str,
    json_body: Option<&T>,
) -> Result<(StatusCode, String), KalshiError>
where
    T: serde::Serialize + ?Sized,
{
    let base = base_url.trim_end_matches('/');
    let url = format!("{}{}", base, path);
    let parsed = Url::parse(&url).map_err(|e| KalshiError::Other(e.to_string()))?;
    // The signature covers only the path portion, never the query string.
    let signed_path = parsed.path().to_string();
    let kind = RequestKind::from_method(method.as_str());
    let max_attempts = limiter.map_or(1, |l| l.max_retries() + 1);

    let mut attempt = 0;
    loop {
        if let Some(l) = limiter {
            l.acquire(kind, DEFAULT_REQUEST_COST).await;
        }

        let mut request = http_client.request(method.clone(), parsed.as_str());
        if let Some(account) = account {
            let (key_id, timestamp, signature) =
                create_auth_headers(account, method.as_str(), &signed_path)?;
            request = request
                .header("KALSHI-ACCESS-KEY", key_id)
                .header("KALSHI-ACCESS-TIMESTAMP", &timestamp)
                .header("KALSHI-ACCESS-SIGNATURE", signature);
        }
        if let Some(body) = json_body {
            request = request.json(body);
        }

        let resp = request.send().await?;
        let status = resp.status();
        let text = resp.text().await?;

        if status == StatusCode::TOO_MANY_REQUESTS && attempt + 1 < max_attempts {
            if let Some(l) = limiter {
                tokio::time::sleep(l.backoff(attempt)).await;
            }
            attempt += 1;
            continue;
        }
        if !status.is_success() {
            return Err(KalshiError::Other(format!("HTTP {}: {}", status, text)));
        }
        return Ok((status, text));
    }
}
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
    limiter: Option<&RateLimiter>,
    path: &str,
) -> Result<String, KalshiError> {
    let (_, body) = send_request::<()>(
        http_client,
        base_url,
        None,
        limiter,
        Method::GET,
        path,
        None,
    )
    .await?;
    Ok(body)
}

/// Make an authenticated GET request
pub(crate) async fn authenticated_get<T>(
    http_client: &Client,
    base_url: &str,
    account: &Account,
    limiter: Option<&RateLimiter>,
    path: &str,
    json_body: Option<&T>,
) -> Result<String, KalshiError>
where
    T: serde::Serialize + ?Sized,
{
    let (_, body) = send_request(
        http_client,
        base_url,
        Some(account),
        limiter,
        Method::GET,
        path,
        json_body,
    )
    .await?;
    Ok(body)
}

/// Make an authenticated POST request
pub(crate) async fn authenticated_post<T>(
    http_client: &Client,
    base_url: &str,
    account: &Account,
    limiter: Option<&RateLimiter>,
    path: &str,
    json_body: Option<&T>,
) -> Result<String, KalshiError>
where
    T: serde::Serialize + ?Sized,
{
    let (_, body) = send_request(
        http_client,
        base_url,
        Some(account),
        limiter,
        Method::POST,
        path,
        json_body,
    )
    .await?;
    Ok(body)
}

///make an authenticated put request
pub(crate) async fn authenticated_put<T>(
    http_client: &Client,
    base_url: &str,
    account: &Account,
    limiter: Option<&RateLimiter>,
    path: &str,
    json_body: Option<&T>,
) -> Result<(StatusCode, String), KalshiError>
where
    T: serde::Serialize + ?Sized,
{
    send_request(
        http_client,
        base_url,
        Some(account),
        limiter,
        Method::PUT,
        path,
        json_body,
    )
    .await
}

/// Make an authenticated DELETE request
pub(crate) async fn authenticated_delete<T>(
    http_client: &Client,
    base_url: &str,
    account: &Account,
    limiter: Option<&RateLimiter>,
    path: &str,
    body: Option<&T>,
) -> Result<(StatusCode, String), KalshiError>
where
    T: serde::Serialize + ?Sized,
{
    send_request(
        http_client,
        base_url,
        Some(account),
        limiter,
        Method::DELETE,
        path,
        body,
    )
    .await
}

///method to convert strings to utc timestamps.. pretty useful for the responses we get back
pub(crate) fn str_to_utc(timestamp: &str) -> DateTime<Utc> {
    DateTime::parse_from_str(timestamp, "%Y-%m-%dT%H:%M:%SZ")
        .expect("Failed to parse from str to utc... string might not be in utc")
        .with_timezone(&Utc)
}
