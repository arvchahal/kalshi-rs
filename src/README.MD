# Kalshi Rust SDK - Source Code Structure

This document explains how the repository is organized and how the different modules work together.


## Repository Architecture

The SDK follows a clean, modular architecture that mirrors the Kalshi API structure:

```
src/
 lib.rs                    # Main entry point, re-exports
 client.rs                 # KalshiClient - main HTTP client
 auth/                     # Authentication module
      mod.rs
      auth_loader.rs       # PEM loading & request signing
      models.rs            # Account struct
  helpers.rs                # Internal HTTP helpers
  errors.rs                 # Error types
  markets/                  # Markets endpoints
      mod.rs
      endpoints.rs         # HTTP methods for markets
      models.rs            # Market structs (Market, MarketsQuery, etc.)
  portfolio/                # Portfolio & trading endpoints
      mod.rs
      endpoints.rs         # HTTP methods for orders/positions
      models.rs            # Order, Position structs
  events/                   # Events endpoints
  exchange/                 # Exchange status endpoints
  series/                   # Series endpoints
  communications/           # RFQ & quotes endpoints
  api_keys/                 # API key management
  milestones/              # Milestones endpoints
  multivariate_collections/ # Multivariate collections
  structured_targets/       # Structured targets
```


## Core Design Patterns

### Pattern 1: Module Organization (endpoints.rs + models.rs)

Every API domain (markets, portfolio, events, etc.) follows the same structure:

#### **endpoints.rs** - HTTP Methods

Contains all the HTTP endpoint implementations as methods on `KalshiClient`:

```rust
// src/markets/endpoints.rs
use crate::client::KalshiClient;

const GET_MARKETS: &str = "/trade-api/v2/markets";
const GET_MARKET: &str = "/trade-api/v2/markets/{}";

impl KalshiClient {
    pub async fn get_all_markets(
        &self,
        params: &MarketsQuery,
    ) -> Result<GetMarketsResponse, KalshiError> {
        // Build URL with query params
        let query = serde_urlencoded::to_string(&params)?;
        let url = format!("{}?{}", GET_MARKETS, query);

        // Make HTTP request
        let resp = self.unauthenticated_get(&url).await?;

        // Parse JSON response
        let data: GetMarketsResponse = serde_json::from_str(&resp)?;
        Ok(data)
    }
}
```

**Key points:**
- URL path constants at the top
- Each endpoint is a method on `KalshiClient`
- Methods call helpers like `authenticated_get()`, `authenticated_post()`, etc.
- Response JSON is deserialized into strongly-typed structs


#### **models.rs** - Data Structures

Contains all the structs for that domain:

```rust
// src/markets/models.rs
use serde::{Deserialize, Serialize};

// Query parameters (for filtering list endpoints)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarketsQuery {
    pub limit: Option<u16>,
    pub cursor: Option<String>,
    pub status: Option<String>,
    pub event_ticker: Option<String>,
    // ... more optional filters
}

// Response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMarketsResponse {
    pub markets: Vec<Market>,
    pub cursor: Option<String>,  // For pagination
}

// Core domain model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub ticker: String,
    pub title: String,
    pub status: String,
    pub yes_bid: u32,
    pub yes_ask: u32,
    // ... many more fields
}
```

**Three types of structs:**

1. **Query Structs** (`*Query`) - URL query parameters for filtering
   - Use `Option<T>` for all fields (optional filtering)
   - Derive `Serialize` (for URL encoding) and `Default`
   - Examples: `MarketsQuery`, `EventsQuery`, `GetOrdersParams`

2. **Request Structs** (`*Request`) - JSON request bodies
   - Used for creating/modifying resources
   - Some fields required, others `Option<T>`
   - Examples: `CreateOrderRequest`, `AmendOrderRequest`

3. **Response Structs** (`Get*Response` and domain models)
   - Deserialize JSON responses from API
   - Single item: `GetMarketResponse { market: Market }`
   - Lists: `GetMarketsResponse { markets: Vec<Market>, cursor: Option<String> }`


### Pattern 2: Client Architecture (client.rs + helpers.rs)

#### **client.rs** - Main HTTP Client

```rust
// src/client.rs
pub struct KalshiClient {
    pub(crate) http_client: Client,        // reqwest client
    pub(crate) account: Account,           // Auth credentials
    pub(crate) base_url: String,           // API base URL
}

impl KalshiClient {
    pub fn new(user: Account) -> KalshiClient {
        KalshiClient {
            http_client: Client::new(),
            account: user,
            base_url: "https://api.elections.kalshi.com".to_string(),
        }
    }

    // Wrapper methods that delegate to helpers
    pub async fn authenticated_get<T>(...) -> Result<String, KalshiError> {
        helpers::authenticated_get(&self.http_client, &self.base_url, &self.account, path, body).await
    }

    pub async fn authenticated_post<T>(...) -> Result<String, KalshiError> {
        helpers::authenticated_post(&self.http_client, &self.base_url, &self.account, path, body).await
    }

    // ... more wrappers for DELETE, PUT, unauthenticated GET
}
```


#### **helpers.rs** - HTTP Request Implementation

Internal module that handles the actual HTTP logic:

```rust
// src/helpers.rs

// Creates auth headers (key_id, timestamp, signature)
pub(crate) fn create_auth_headers(
    account: &Account,
    method: &str,
    path: &str,
) -> Result<(String, String, String), KalshiError> {
    let timestamp = get_current_timestamp_ms();
    let signature = sign_request(account.private_key_pem(), method, path, timestamp)?;
    Ok((account.key_id().to_string(), timestamp, signature))
}

// Makes authenticated GET request
pub(crate) async fn authenticated_get<T>(
    http_client: &Client,
    base_url: &str,
    account: &Account,
    path: &str,
    json_body: Option<&T>,
) -> Result<String, KalshiError> {
    let url = format!("{}{}", base_url, path);

    // Parse URL to extract path for signing
    let parsed = Url::parse(&url)?;
    let signed_path = parsed.path().to_string();

    // Create auth headers
    let (key_id, timestamp, signature) = create_auth_headers(account, "GET", &signed_path)?;

    // Build request with auth headers
    let mut request = http_client
        .get(parsed.as_str())
        .header("KALSHI-ACCESS-KEY", key_id)
        .header("KALSHI-ACCESS-TIMESTAMP", timestamp)
        .header("KALSHI-ACCESS-SIGNATURE", signature);

    if let Some(body) = json_body {
        request = request.json(body);
    }

    // Send request and handle response
    let resp = request.send().await?;
    let status = resp.status();
    let body = resp.text().await?;

    if !status.is_success() {
        return Err(KalshiError::Other(format!("HTTP {}: {}", status, body)));
    }

    Ok(body)
}

// Similar implementations for POST, PUT, DELETE, and unauthenticated GET
```

**Why this separation?**
- `client.rs` provides the public API surface
- `helpers.rs` contains the complex HTTP/auth logic
- Endpoint modules just call the wrappers without worrying about HTTP details


### Pattern 3: Authentication (auth/)

#### **auth/models.rs** - Account Struct

```rust
#[derive(Debug, Clone)]
pub struct Account {
    private_key_pem: String,  // Private key for signing
    key_id: String,           // API key ID
}

impl Account {
    pub fn new(private_key_pem: String, key_id: String) -> Self {
        Self { private_key_pem, key_id }
    }

    pub fn from_file(path: &str, key_id: impl Into<String>) -> std::io::Result<Self> {
        let private_key_pem = std::fs::read_to_string(path)?;
        Ok(Self::new(private_key_pem, key_id.into()))
    }

    pub fn private_key_pem(&self) -> &str { &self.private_key_pem }
    pub fn key_id(&self) -> &str { &self.key_id }
}
```


#### **auth/auth_loader.rs** - Request Signing

Handles RSA-PSS signature generation for authenticated requests:

```rust
pub fn sign_request(
    private_key_pem: &str,
    method: &str,
    path: &str,
    timestamp: u64,
) -> Result<String, Box<dyn std::error::Error>> {
    // Format: timestamp + method + path
    let msg_string = format!("{}{}{}", timestamp, method, path);

    // Parse private key (supports both PKCS1 and PKCS8)
    let private_key = if private_key_pem.contains("BEGIN RSA PRIVATE KEY") {
        RsaPrivateKey::from_pkcs1_pem(private_key_pem)?
    } else {
        RsaPrivateKey::from_pkcs8_pem(private_key_pem)?
    };

    // Sign with PSS padding (32 byte salt length per Kalshi spec)
    let signing_key = SigningKey::<Sha256>::new_with_salt_len(private_key, 32);
    let signature = signing_key.sign_with_rng(&mut thread_rng(), msg_string.as_bytes());

    // Base64 encode signature
    let sig_b64 = BASE64.encode(signature.to_bytes());
    Ok(sig_b64)
}
```

**Kalshi Authentication Flow:**

1. Generate timestamp (milliseconds since epoch)
2. Create message string: `timestamp + method + path`
3. Sign message with RSA private key (PSS padding, SHA256, 32 byte salt)
4. Base64 encode signature
5. Send in headers:
   - `KALSHI-ACCESS-KEY`: your API key ID
   - `KALSHI-ACCESS-TIMESTAMP`: the timestamp
   - `KALSHI-ACCESS-SIGNATURE`: the base64 signature


## How Endpoints Work

Let's trace a full request through the codebase:

### Example: `client.get_all_markets(&params)`

1. **User calls method** on `KalshiClient`:
   ```rust
   let params = MarketsQuery { status: Some("active".into()), ..Default::default() };
   let response = client.get_all_markets(&params).await?;
   ```

2. **Endpoint method** in `markets/endpoints.rs`:
   ```rust
   pub async fn get_all_markets(&self, params: &MarketsQuery) -> Result<GetMarketsResponse, KalshiError> {
       // Serialize params to URL query string
       let query = serde_urlencoded::to_string(&params)?;
       let url = format!("{}?{}", GET_MARKETS, query);

       // Call unauthenticated helper (markets endpoint is public)
       let resp = self.unauthenticated_get(&url).await?;

       // Deserialize JSON response
       let data: GetMarketsResponse = serde_json::from_str(&resp)?;
       Ok(data)
   }
   ```

3. **Client wrapper** in `client.rs`:
   ```rust
   pub async fn unauthenticated_get(&self, path: &str) -> Result<String, KalshiError> {
       helpers::unauthenticated_get(&self.http_client, &self.base_url, path).await
   }
   ```

4. **HTTP helper** in `helpers.rs`:
   ```rust
   pub(crate) async fn unauthenticated_get(
       http_client: &Client,
       base_url: &str,
       path: &str,
   ) -> Result<String, KalshiError> {
       let url = format!("{}{}", base_url.trim_end_matches('/'), path);
       let resp = http_client.get(&url).send().await?;
       let status = resp.status();
       let body = resp.text().await?;

       if !status.is_success() {
           return Err(KalshiError::Other(format!("HTTP {}: {}", status, body)));
       }

       Ok(body)
   }
   ```

5. **Response returned** and deserialized into `GetMarketsResponse`


### Example: `client.create_order(&order_request)` (Authenticated)

1. **User creates request**:
   ```rust
   let order = CreateOrderRequest {
       ticker: "TICKER".into(),
       action: "buy".into(),
       side: "yes".into(),
       count: 1,
       yes_price: Some(50),
       ..Default::default()
   };
   let response = client.create_order(&order).await?;
   ```

2. **Endpoint method** in `portfolio/endpoints.rs`:
   ```rust
   pub async fn create_order(&self, body: &CreateOrderRequest) -> Result<CreateOrderResponse, KalshiError> {
       // Call authenticated POST helper
       let resp = self.authenticated_post(CREATE_ORDER, Some(&body)).await?;

       // Deserialize response
       let data: CreateOrderResponse = serde_json::from_str(&resp)?;
       Ok(data)
   }
   ```

3. **Client wrapper** in `client.rs`:
   ```rust
   pub async fn authenticated_post<T>(&self, path: &str, json_body: Option<&T>) -> Result<String, KalshiError> {
       helpers::authenticated_post(&self.http_client, &self.base_url, &self.account, path, json_body).await
   }
   ```

4. **HTTP helper with auth** in `helpers.rs`:
   ```rust
   pub(crate) async fn authenticated_post<T>(
       http_client: &Client,
       base_url: &str,
       account: &Account,
       path: &str,
       json_body: Option<&T>,
   ) -> Result<String, KalshiError> {
       let url = format!("{}{}", base_url, path);
       let parsed = Url::parse(&url)?;
       let signed_path = parsed.path().to_string();

       // Generate auth headers (key_id, timestamp, signature)
       let (key_id, timestamp, signature) = create_auth_headers(account, "POST", &signed_path)?;

       // Build request with auth headers
       let mut request = http_client
           .post(parsed.as_str())
           .header("KALSHI-ACCESS-KEY", key_id)
           .header("KALSHI-ACCESS-TIMESTAMP", timestamp)
           .header("KALSHI-ACCESS-SIGNATURE", signature);

       if let Some(body) = json_body {
           request = request.json(body);
       }

       let resp = request.send().await?;
       let body = resp.text().await?;
       Ok(body)
   }
   ```

5. **Auth headers created** by `create_auth_headers()` which calls `sign_request()`

6. **Response returned** and deserialized into `CreateOrderResponse`


## Key Implementation Details

### URL Building

```rust
// Base URL (trimmed to avoid double slashes)
const base_url = "https://api.elections.kalshi.com";

// Path replacement for dynamic segments
let path = GET_MARKET.replace("{}", ticker);  // "/trade-api/v2/markets/TICKER"

// Query string building
let query = serde_urlencoded::to_string(&params)?;
let url = if query.is_empty() {
    path.to_string()
} else {
    format!("{}?{}", path, query)
};

// Final URL: https://api.elections.kalshi.com/trade-api/v2/markets/TICKER?limit=10&status=active
```


### Response Parsing

```rust
// All responses are JSON strings from HTTP helpers
let resp: String = self.authenticated_get(path, None).await?;

// Deserialize with serde_json
let data: GetMarketResponse = serde_json::from_str(&resp)
    .map_err(|e| KalshiError::Other(format!("Parse error: {e}. Response: {resp}")))?;

// Return strongly-typed struct
Ok(data)
```


### Error Handling

```rust
// src/errors.rs
#[derive(Debug)]
pub enum KalshiError {
    HttpError(reqwest::Error),
    Other(String),
}

// Automatic conversion from reqwest errors
impl From<reqwest::Error> for KalshiError {
    fn from(err: reqwest::Error) -> Self {
        KalshiError::HttpError(err)
    }
}

// Used throughout endpoints
if !status.is_success() {
    return Err(KalshiError::Other(format!("HTTP {}: {}", status, body)));
}
```


## Module Summary

| Module | Purpose | Exports |
|--------|---------|---------|
| **lib.rs** | Entry point | `KalshiClient`, `Account`, all endpoint modules |
| **client.rs** | HTTP client | `KalshiClient` struct + HTTP wrapper methods |
| **helpers.rs** | HTTP implementation | Internal `authenticated_*` and `unauthenticated_*` functions |
| **auth/** | Authentication | `Account` struct, PEM loading, request signing |
| **errors.rs** | Error types | `KalshiError` enum |
| **markets/** | Market data | `get_all_markets()`, `get_market()`, `get_trades()`, etc. + models |
| **portfolio/** | Orders & positions | `create_order()`, `get_balance()`, `get_positions()`, etc. + models |
| **events/** | Events | `get_all_events()`, `get_event()`, `get_event_metadata()` + models |
| **exchange/** | Exchange status | `get_exchange_status()`, `get_exchange_schedule()` + models |
| **series/** | Series | `get_all_series()`, `get_series_by_ticker()` + models |
| **communications/** | RFQs & quotes | `create_rfq()`, `get_quotes()`, `accept_quote()` + models |
| **api_keys/** | API key mgmt | `get_api_keys()`, `generate_api_key()`, `delete_api_key()` + models |
| **milestones/** | Milestones | `get_milestones()`, `get_milestone()` + models |
| **multivariate_collections/** | Multivariate data | `get_multivariate_event_collections()` + models |
| **structured_targets/** | Structured targets | `get_all_structured_targets()` + models |


## Testing

Tests are organized in `tests/` directory, mirroring the source structure:

```
tests/
  common/mod.rs              # Shared test utilities
  auth/auth_loader_test.rs   # Auth tests
  markets_test/              # Markets endpoint tests
      mod.rs
      endpoints_test.rs
      models_test.rs
  portfolio_test/            # Portfolio tests
  ...                        # Other module tests
```

Tests use real API credentials from environment variables and make actual HTTP requests.


## Adding New Endpoints

To add a new endpoint:

1. **Add const path** at top of `endpoints.rs`:
   ```rust
   const NEW_ENDPOINT: &str = "/trade-api/v2/new/endpoint";
   ```

2. **Add models** to `models.rs`:
   ```rust
   #[derive(Debug, Serialize, Deserialize)]
   pub struct NewRequest { /* fields */ }

   #[derive(Debug, Serialize, Deserialize)]
   pub struct NewResponse { /* fields */ }
   ```

3. **Implement method** on `KalshiClient` in `endpoints.rs`:
   ```rust
   impl KalshiClient {
       pub async fn new_endpoint(&self, req: &NewRequest) -> Result<NewResponse, KalshiError> {
           let resp = self.authenticated_post(NEW_ENDPOINT, Some(&req)).await?;
           let data: NewResponse = serde_json::from_str(&resp)?;
           Ok(data)
       }
   }
   ```

4. **Export** in `mod.rs` if needed

5. **Add tests** in `tests/` directory


## Summary

The Kalshi Rust SDK is structured around:

- **Modularity**: Each API domain is a separate module (markets, portfolio, etc.)
- **Consistency**: Every module follows endpoints.rs + models.rs pattern
- **Type Safety**: All requests/responses are strongly typed
- **Testability**: Integration tests mirror source structure

This architecture makes the SDK easy to understand, extend, and maintain! Took a lot of inspiration from my time at Splunk in designing this repo.. as for a linter sadly the rust linters suck and as such I will use 2 lines of white space after each funciton and a new line after complex blocks of code...
