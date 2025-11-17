//! Auth module models.
//!
//! This module contains data structures for auth functionality.

/// Account credentials for Kalshi API authentication
///
/// Stores the private key PEM and API key ID needed for request signing
#[derive(Debug, Clone)]


pub struct Account {
    private_key_pem: String,
    key_id: String,
}


impl Account {
    /// Create a new Account directly with credentials
    ///
    /// # Example
    /// ```no_run
    /// use kalshi_rs::auth::Account;
    ///
    pub fn new(private_key_pem: String, key_id: String) -> Self {
        Self { private_key_pem, key_id }
    }


    /// Load private key from a file path with API key ID
    ///
    /// Accepts both relative and absolute paths:
    /// - Relative: "kalshi_private.pem", "keys/my_key.pem"
    /// - Absolute: "/Users/name/.config/kalshi/key.pem"

    pub fn from_file(path: &str, key_id: impl Into<String>) -> std::io::Result<Self> {
        let private_key_pem = std::fs::read_to_string(path)?;
        Ok(Self::new(private_key_pem, key_id.into()))
    }


    /// Get reference to the private key PEM string
    pub fn private_key_pem(&self) -> &str {
        &self.private_key_pem
    }


    /// Get reference to the API key ID
    pub fn key_id(&self) -> &str {
        &self.key_id
    }
}
