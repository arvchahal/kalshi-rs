# Kalshi Rust SDK - Quickstart Guide

This guide covers all endpoints, request/response patterns, and how to use the SDK effectively.


## Table of Contents

1. [Setup & Authentication](#setup--authentication)
2. [Understanding Request/Response Patterns](#understanding-requestresponse-patterns)
3. [All Available Endpoints](#all-available-endpoints)
4. [Common Usage Patterns](#common-usage-patterns)


## Setup & Authentication

### Installation

```bash
cargo add kalshi-rs
```

### Authentication Setup

The SDK requires two pieces of authentication:
1. **API Key ID** - Your public API key identifier
2. **Private Key PEM file** - Your private key for signing requests

```rust
use kalshi_rs::{KalshiClient, Account};

// Method 1: Load from file
let api_key_id = "your-api-key-id";
let account = Account::from_file("kalshi_private.pem", api_key_id)?;

// Method 2: Load from string
let private_key_pem = std::fs::read_to_string("kalshi_private.pem")?;
let account = Account::new(private_key_pem, api_key_id.to_string());

// Create the client
let client = KalshiClient::new(account);
```

Get your API credentials from: https://docs.kalshi.com/introduction/#authentication


## Understanding Request/Response Patterns

The SDK follows a consistent pattern for all endpoints:

### Pattern 1: Query Parameters (`*Query` structs)

Used for filtering list endpoints like `get_all_markets()`, `get_events()`, etc.

```rust
use kalshi_rs::markets::models::MarketsQuery;

let params = MarketsQuery {
    limit: Some(10),                           // Optional: max results
    cursor: None,                              // Optional: pagination cursor
    status: Some("active".to_string()),        // Optional: filter by status
    event_ticker: Some("KXINFL".to_string()),  // Optional: filter by event
    ..Default::default()                       // Use defaults for other fields
};

let response = client.get_all_markets(&params).await?;
```

**All Query structs use `Option<T>` for optional filtering** - use `None` or omit fields to skip filtering.


### Pattern 2: Request Bodies (`*Request` structs)

Used for creating/modifying resources like orders, RFQs, etc.

```rust
use kalshi_rs::portfolio::models::CreateOrderRequest;

let order_request = CreateOrderRequest {
    ticker: "KXWTAMATCH".to_string(),
    action: "buy".to_string(),               // "buy" or "sell"
    side: "yes".to_string(),                 // "yes" or "no"
    count: 1,                                 // Number of contracts
    type_: Some("limit".to_string()),        // Order type
    yes_price: Some(50),                     // Price in cents
    ..Default::default()
};

let response = client.create_order(&order_request).await?;
```


### Pattern 3: Response Types (`Get*Response` structs)

All responses are strongly typed and match Kalshi's API structure.

```rust
// Single item responses wrap the item
pub struct GetMarketResponse {
    pub market: Market,  // The actual market data
}

// List responses include the items + optional cursor for pagination
pub struct GetMarketsResponse {
    pub markets: Vec<Market>,       // List of markets
    pub cursor: Option<String>,     // Next page cursor if more results exist
}

// Usage
let response = client.get_market("KXINFL-24DEC-T3.0").await?;
println!("Market title: {}", response.market.title);

let markets_response = client.get_all_markets(&params).await?;
for market in markets_response.markets {
    println!("{}: {}", market.ticker, market.title);
}
```


### Pattern 4: Path Parameters

Some endpoints need IDs/tickers in the URL path - these are function arguments:

```rust
// Get specific market by ticker
client.get_market("KXINFL-24DEC-T3.0").await?;

// Get specific order by order_id
client.get_order("order-uuid-here").await?;

// Cancel order by order_id
client.cancel_order("order-uuid-here".to_string()).await?;
```


## All Available Endpoints

The SDK implements **58 endpoints** across **10 modules**. All methods are called on `KalshiClient`.


### Markets Module (Public - No Auth Required)

Market data, orderbooks, trades, and candlesticks.

```rust
// Get all markets with filtering
let params = MarketsQuery {
    status: Some("active".into()),
    limit: Some(10),
    ..Default::default()
};
let markets = client.get_all_markets(&params).await?;

// Get specific market
let market = client.get_market("KXINFL-24DEC-T3.0").await?;

// Get trade history
let trades = client.get_trades(
    Some(100),                    // limit
    None,                         // cursor
    Some("KXINFL-24DEC".into()),  // ticker filter
    None,                         // min_ts
    None,                         // max_ts
).await?;

// Get orderbook with depth
let orderbook = client.get_market_orderbook("KXINFL-24DEC-T3.0", Some(10)).await?;

// Get candlesticks for price charts
let candles = client.get_market_candlesticks(
    "KXINFL",                     // series_ticker
    "KXINFL-24DEC-T3.0",          // market_ticker
    1704067200,                   // start_ts (unix timestamp)
    1704153600,                   // end_ts
    3600,                         // period_interval (seconds)
).await?;
```


### Portfolio Module (Authenticated)

Orders, positions, fills, balances, and order groups.

#### Order Management

```rust
// Create order
let order = CreateOrderRequest {
    ticker: "KXINFL-24DEC-T3.0".into(),
    action: "buy".into(),
    side: "yes".into(),
    count: 5,
    type_: Some("limit".into()),
    yes_price: Some(50),  // 50 cents
    ..Default::default()
};
let response = client.create_order(&order).await?;

// Get all orders with filtering
let params = GetOrdersParams {
    limit: Some(10),
    ticker: Some("KXINFL-24DEC-T3.0".into()),
    status: Some("resting".into()),  // "resting", "filled", "canceled", etc.
    ..Default::default()
};
let orders = client.get_orders(&params).await?;

// Get specific order
let order = client.get_order("order-id-here").await?;

// Cancel order
client.cancel_order("order-id-here".to_string()).await?;

// Amend order (change price/quantity)
let amend = AmendOrderRequest {
    yes_price: Some(55),
    count: Some(3),
    ..Default::default()
};
client.amend_order("order-id-here", &amend).await?;

// Decrease order quantity
let decrease = DecreaseOrderRequest { count: 2 };
client.decrease_order("order-id-here", &decrease).await?;
```

#### Batch Operations

```rust
// Batch create multiple orders
let batch = BatchCreateOrdersRequest {
    orders: vec![order1, order2, order3],
};
client.batch_create_orders(&batch).await?;

// Batch cancel multiple orders
let cancel_batch = BatchCancelOrdersRequest {
    order_ids: vec!["id1".into(), "id2".into()],
};
client.batch_cancel_orders(&cancel_batch).await?;
```

#### Order Groups (Conditional Orders)

```rust
// Create order group
let group = CreateOrderGroupRequest {
    group_name: "my-strategy".to_string(),
    orders: vec![order1, order2],
};
client.create_order_group(&group).await?;

// Get all order groups
let groups = client.get_order_groups().await?;

// Get specific order group
let group = client.get_order_group("group-id").await?;

// Reset order group (cancel all orders in group)
client.reset_order_group("group-id").await?;

// Delete order group
client.delete_order_group("group-id").await?;
```

#### Portfolio Information

```rust
// Get account balance
let balance = client.get_balance().await?;
println!("Available: ${}", balance.balance);

// Get all positions
let params = GetPositionsParams {
    limit: Some(50),
    event_ticker: Some("KXINFL".into()),
    ..Default::default()
};
let positions = client.get_positions(&params).await?;

// Get fills (executed trades)
let params = GetFillsParams {
    limit: Some(100),
    ticker: Some("KXINFL-24DEC-T3.0".into()),
    ..Default::default()
};
let fills = client.get_fills(&params).await?;

// Get settlements
let params = GetSettlementsParams {
    limit: Some(50),
    ..Default::default()
};
let settlements = client.get_settlements(&params).await?;
```

#### Queue Position (Advanced)

```rust
// Get single order queue position
let queue_pos = client.get_order_queue_position("order-id").await?;

// Get multiple order queue positions
let params = GetQueueParams {
    order_ids: vec!["id1".into(), "id2".into()],
};
let queue_positions = client.get_queue_positions(&params).await?;
```


### Events Module (Public - No Auth Required)

Event information and metadata.

```rust
// Get all events
let params = EventsQuery {
    limit: Some(20),
    status: Some("open".into()),
    series_ticker: Some("KXINFL".into()),
    ..Default::default()
};
let events = client.get_all_events(&params).await?;

// Get specific event
let event = client.get_event("KXINFL-24DEC").await?;

// Get event metadata (images, settlement sources, etc.)
let metadata = client.get_event_metadata("KXINFL-24DEC").await?;
```


###  Exchange Module (Public - No Auth Required)

Exchange status, schedule, and announcements.

```rust
// Get exchange status (open/closed)
let status = client.get_exchange_status().await?;
println!("Trading: {}", status.trading_active);

// Get exchange schedule
let schedule = client.get_exchange_schedule().await?;

// Get announcements
let announcements = client.get_exchange_announcements().await?;

// Get user data timestamp (for streaming APIs)
let timestamp = client.get_user_data_timestamp().await?;
```


###  Series Module (Public - No Auth Required)

Series information (collections of related events).

```rust
// Get all series
let params = SeriesQuery {
    limit: Some(50),
    cursor: None,
};
let series = client.get_all_series(&params).await?;

// Get specific series
let series = client.get_series_by_ticker("KXINFL").await?;
```


### Communications Module (Authenticated)

RFQs (Request for Quote) and quote management for negotiated trading.

```rust
// Create RFQ
let rfq = CreateRfqRequest {
    tickers: vec!["KXINFL-24DEC-T3.0".into()],
    quantity: 10,
    side: "yes".into(),
};
let created = client.create_rfq(&rfq).await?;

// Get all RFQs
let rfqs = client.get_rfqs().await?;

// Get specific RFQ
let rfq = client.get_rfq("rfq-id").await?;

// Delete RFQ
client.delete_rfq("rfq-id").await?;

// Create quote (response to RFQ)
let quote = CreateQuoteRequest {
    rfq_id: "rfq-id".to_string(),
    price: 52,
    side: "yes".into(),
};
client.create_quote(&quote).await?;

// Get all quotes
let params = GetQuotesParams {
    limit: Some(20),
    status: Some("pending".into()),
    ..Default::default()
};
let quotes = client.get_quotes(&params).await?;

// Get specific quote
let quote = client.get_quote("quote-id").await?;

// Accept quote
let accept = AcceptQuoteRequest {
    accepted_side: "yes".into(),
};
client.accept_quote("quote-id", &accept).await?;

// Confirm quote
client.confirm_quote("quote-id").await?;

// Delete quote
client.delete_quote("quote-id").await?;

// Get communications ID
let comm_id = client.get_communications_id().await?;
```


### API Keys Module (Authenticated)

Manage API keys programmatically.

```rust
// List all API keys
let keys = client.get_api_keys().await?;

// Generate new API key
let request = GenerateApiKeyRequest {
    name: "my-trading-bot".to_string(),
};
let new_key = client.generate_api_key(&request).await?;
println!("New key ID: {}", new_key.api_key.key_id);

// Delete API key
client.delete_api_key("key-id-to-delete").await?;
```


### <� Milestones Module (Public - No Auth Required)

User milestones and achievements.

```rust
// Get all milestones
let milestones = client.get_milestones(Some(50)).await?;

// Get specific milestone
let milestone = client.get_milestone("milestone-id").await?;
```


### Multivariate Collections Module (Public - No Auth Required)

Collections of related multivariate events.

```rust
// Get all multivariate event collections
let collections = client.get_multivariate_event_collections().await?;

// Get specific collection
let collection = client.get_multivariate_event_collection("collection-ticker").await?;
```


### Structured Targets Module (Public - No Auth Required)

Structured target markets information.

```rust
// Get all structured targets
let params = StructuredTargetsQuery {
    limit: Some(20),
    cursor: None,
};
let targets = client.get_all_structured_targets(&params).await?;

// Get specific structured target
let target = client.get_structured_target("target-id").await?;
```


## Common Usage Patterns

### Pagination

Most list endpoints support pagination with `limit` and `cursor`:

```rust
let mut all_markets = Vec::new();
let mut cursor = None;

loop {
    let params = MarketsQuery {
        limit: Some(100),
        cursor: cursor.clone(),
        status: Some("active".into()),
        ..Default::default()
    };

    let response = client.get_all_markets(&params).await?;
    all_markets.extend(response.markets);

    // Check if there are more pages
    if response.cursor.is_none() {
        break;
    }
    cursor = response.cursor;
}

println!("Fetched {} total markets", all_markets.len());
```


### Error Handling

All endpoints return `Result<T, KalshiError>`:

```rust
use kalshi_rs::errors::KalshiError;

match client.get_market("INVALID-TICKER").await {
    Ok(response) => println!("Market: {}", response.market.title),
    Err(KalshiError::Other(msg)) => eprintln!("API error: {}", msg),
    Err(e) => eprintln!("Error: {:?}", e),
}
```


### Using Default for Optional Fields

Most request/query structs derive `Default`, so you can use the spread operator:

```rust
// Only set the fields you care about
let params = MarketsQuery {
    limit: Some(10),
    status: Some("active".into()),
    ..Default::default()  // All other fields are None/default
};
```


### Working with Prices

Kalshi uses **cents** for all price values:

```rust
// Create order at 52 cents (52% probability)
let order = CreateOrderRequest {
    yes_price: Some(52),  // 52 cents = $0.52
    ..Default::default()
};

// Markets also use cents
let market = client.get_market("TICKER").await?;
println!("Yes bid: {} cents (${:.2})",
    market.market.yes_bid,
    market.market.yes_bid as f64 / 100.0
);
```


### Async/Await Pattern

All SDK methods are async and require an async runtime:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KalshiClient::new(account);

    // All methods are async
    let markets = client.get_all_markets(&params).await?;
    let balance = client.get_balance().await?;

    Ok(())
}
```


## Next Steps

- **Examples**: Check the `examples/` directory for complete working examples
- **API Reference**: See the full API documentation at https://docs.kalshi.com/
- **Source Code**: Browse the SDK source at https://github.com/arvchahal/kalshi-rs
- **Tests**: Clone the repo and Run `cargo test` to see integration tests with real API calls but you will need to add your own private key and api key id and their paths by exporting them. see setup_client method for more


## Summary

- **58 endpoints** across 10 modules
- **28 public endpoints** (no authentication required)
- **30 authenticated endpoints** (require API key)
- Consistent `*Query`, `*Request`, and `*Response` patterns
- Strong typing for all parameters and responses
- Built-in pagination support
- Comprehensive error handling

Happy trading! 
