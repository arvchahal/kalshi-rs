use kalshi_rust_sdk::{KalshiClient};
use kalshi_rust_sdk::auth::auth_loader::load_auth_from_file;

pub fn setup_client() -> KalshiClient {
    let account = load_auth_from_file().expect("Failed to load auth credentials");
    KalshiClient::new(account)
}