// Kalshi Rust SDK
// Unofficial SDK for interacting with the Kalshi trading API
// Provides authentication, market data, portfolio management, and trading functionality


// Core modules
pub mod auth;           // Authentication and credential management
pub mod client;         // Main HTTP client
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


// Re-exports for convenient access
pub use auth::Account;
pub use client::KalshiClient;
