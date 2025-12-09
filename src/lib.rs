//! Kalshi Rust SDK
//!
//! Unofficial Rust SDK for interacting with the Kalshi trading API.
//! Provides authentication, market data retrieval, portfolio management, and trading functionality.
//!
//! # Quick Start
//!
//! ```no_run
//! use kalshi_rs::{Account, KalshiClient};
//! use kalshi_rs::markets::models::MarketsQuery;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // 1. Load your API credentials
//! let account = Account::from_file("kalshi_private.pem", "your-api-key-id")?;
//!
//! // 2. Create a client
//! let client = KalshiClient::new(account);
//!
//! // 3. Use the client to call API endpoints
//! let markets = client.get_all_markets(&MarketsQuery {
//!     limit: Some(10),
//!     status: Some("open".to_string()),
//!     ..Default::default()
//! }).await?;
//!
//! println!("Found {} markets", markets.markets.len());
//! # Ok(())
//! # }
//! ```
//!
//! # Main Components
//!
//! - [`KalshiClient`] - Main client with all API endpoint methods
//! - [`Account`] - Authentication credentials
//!
//! # API Endpoint Modules
//!
//! - [`markets`] - Market data, orderbooks, candlesticks, trades
//! - [`portfolio`] - Orders, positions, fills, balance
//! - [`exchange`] - Exchange status and schedule
//! - [`events`] - Event information
//! - [`series`] - Series data
//!
//! # Finding Endpoint Methods
//!
//! All API endpoint methods are implemented on [`KalshiClient`].
//! Navigate to the [`KalshiClient`] documentation to see all available methods organized by category.


// Core modules
pub mod auth;           // Authentication and credential management
pub mod client;         // Main HTTP client
pub mod ws_client;         // Main Websocket client
pub mod errors;         // Error types
pub(crate) mod helpers; // Internal HTTP helpers


// API endpoint modules
pub mod api_keys;                   // API key management
pub mod communications;             // Announcements and communications
pub mod events;                     // Event data and queries
pub mod exchange;                   // Exchange status and schedule
pub mod markets;                    // Market data and trading
pub mod milestones;                 // Milestone tracking
pub mod multivariate_collections;   // Multivariate event collections
pub mod portfolio;                  // Portfolio and position management
pub mod series;                     // Series data
pub mod structured_targets;         // Structured target markets
pub mod websocket;                  // Websocket trades and orderbook updates


// Re-exports for convenient access
pub use auth::Account;
pub use client::KalshiClient;
pub use ws_client::KalshiWebsocketClient;
