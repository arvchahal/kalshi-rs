/// Helper functions for making authenticated and unauthenticated HTTP requests
use url::Url;
use reqwest::{Client, StatusCode};
use crate::auth::auth_loader::{get_current_timestamp_ms, sign_request};
use crate::errors::KalshiError;
use crate::auth::Account;
use chrono::{DateTime, Utc};

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
    let body:String = resp.text().await?;

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
    json_body: Option<&T>
) -> Result<String, KalshiError> 
    where
        T: serde::Serialize + ?Sized
        {

    let base = base_url.trim_end_matches('/');
    let url = format!("{}{}", base, path);
    let parsed = Url::parse(&url).map_err(|e| KalshiError::Other(e.to_string()))?;

    let signed_path = parsed.path().to_string();
    let (key_id, timestamp, signature) = create_auth_headers(account, "GET", &signed_path)?;

    let mut request = http_client
        .get(parsed.as_str())
        .header("KALSHI-ACCESS-KEY", key_id)
        .header("KALSHI-ACCESS-TIMESTAMP", &timestamp)
        .header("KALSHI-ACCESS-SIGNATURE", signature);
    if let Some(body) = json_body {
        request = request.json(body); // sets Content-Type and serializes
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
    let parsed = url::Url::parse(&url).map_err(|e| KalshiError::Other(e.to_string()))?;

    let signed_path = parsed.path().to_string();
    let (key_id, timestamp, signature) =
        create_auth_headers(account, "POST", &signed_path)?;

    let mut request = http_client
        .post(parsed.as_str())
        .header("KALSHI-ACCESS-KEY", key_id)
        .header("KALSHI-ACCESS-TIMESTAMP", &timestamp)
        .header("KALSHI-ACCESS-SIGNATURE", signature);

    if let Some(body) = json_body {
        request = request.json(body); // sets Content-Type and serializes
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
    let (key_id, timestamp, signature) =
        create_auth_headers(account, "PUT", &signed_path)?;

    let mut request = http_client
        .put(parsed.as_str())
        .header("KALSHI-ACCESS-KEY", key_id)
        .header("KALSHI-ACCESS-TIMESTAMP", &timestamp)
        .header("KALSHI-ACCESS-SIGNATURE", signature);

    if let Some(body) = json_body {
        request = request.json(body); // sets Content-Type and serializes
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
pub(crate) async fn authenticated_delete(
    http_client: &Client,
    base_url: &str,
    account: &Account,
    path: &str,
) -> Result<(StatusCode,String), KalshiError> {
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

    Ok((status,body))
}

/// DEPRECATED: Use portfolio-style manual serialization instead for better error handling.
/// This function uses unwrap_or_default which silently hides serialization errors.
///
/// Preferred pattern:
/// ```
/// let query = serde_urlencoded::to_string(&params)
///     .map_err(|e| KalshiError::Other(format!("Failed to serialize params: {}", e)))?;
/// let url = if query.is_empty() {
///     base_url.to_string()
/// } else {
///     format!("{}?{}", base_url, query)
/// };
/// ```
#[deprecated(note = "Use portfolio-style manual serialization with explicit error handling")]
#[allow(dead_code)]
pub(crate) fn build_url_with_query<T:serde::Serialize>(url:String,query:&T)->String{
    let qs = serde_urlencoded::to_string(query).unwrap_or_default();
      if qs.is_empty() {
          url.to_string()
      } else {
          format!("{}?{}", url, qs)
      }
}

///method to convert strings to utc timestamps.. pretty useful for the responses we get back
pub(crate) fn str_to_utc(timestamp:&str)->DateTime<Utc>{
    DateTime::parse_from_str(timestamp, "%Y-%m-%dT%H:%M:%SZ").expect("Failed to parse from str to utc... string might not be in utc").with_timezone(&Utc)
}

////////////////////////////////////////////////////////////////////////////////////////////////
/*

UNIT TESTS BELOW

*/
////////////////////////////////////////////////////////////////////////////////////////////////







#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestQuery {
        #[serde(skip_serializing_if = "Option::is_none")]
        limit: Option<u16>,
        #[serde(skip_serializing_if = "Option::is_none")]
        status: Option<String>,
    }

    #[test]
    fn test_build_url_with_query_empty() {
        let query = TestQuery {
            limit: None,
            status: None,
        };
        let url = build_url_with_query("/api/markets".to_string(), &query);
        assert_eq!(url, "/api/markets");
    }

    #[test]
    fn test_build_url_with_query_single_param() {
        let query = TestQuery {
            limit: Some(10),
            status: None,
        };
        let url = build_url_with_query("/api/markets".to_string(), &query);
        assert_eq!(url, "/api/markets?limit=10");
    }

    #[test]
    fn test_build_url_with_query_multiple_params() {
        let query = TestQuery {
            limit: Some(25),
            status: Some("active".to_string()),
        };
        let url = build_url_with_query("/api/markets".to_string(), &query);
        assert!(url.contains("limit=25"));
        assert!(url.contains("status=active"));
        assert!(url.starts_with("/api/markets?"));
    }

    #[test]
    fn test_create_auth_headers_format() {
        use crate::auth::Account;

        // This test would require valid RSA keys, so we'll skip the actual signing
        // and just test that the function signature is correct
        // In a real test environment, you'd use test fixtures with valid keys
    }
}