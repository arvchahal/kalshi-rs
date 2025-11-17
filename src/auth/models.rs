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
    /// use kalshi_rust_sdk::auth::Account;
    ///
    /// let account = Account::new(
    ///     "-----BEGIN PRIVATE KEY-----\n...".to_string(),
    ///     "your-api-key-id".to_string()
    /// );
    /// ```
    pub fn new(private_key_pem: String, key_id: String) -> Self {
        Self { private_key_pem, key_id }
    }
    /// Load private key from a file path with API key ID
    ///
    /// Accepts both relative and absolute paths:
    /// - Relative: "kalshi_private.pem", "keys/my_key.pem"
    /// - Absolute: "/Users/name/.config/kalshi/key.pem"
    ///
    /// # Example
    /// ```no_run
    /// use kalshi_rust_sdk::auth::Account;
    ///
    /// let account = Account::from_file(
    ///     "kalshi_private.pem",
    ///     "your-api-key-id"
    /// ).expect("Failed to load key");
    /// ```
    pub fn from_file(path: &str, key_id: impl Into<String>) -> std::io::Result<Self> {
        let private_key_pem = std::fs::read_to_string(path)?;
        Ok(Self::new(private_key_pem, key_id.into()))
    }
    pub fn private_key_pem(&self) -> &str {
        &self.private_key_pem
    }
    pub fn key_id(&self) -> &str {
        &self.key_id
    }
}
