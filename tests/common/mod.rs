use kalshi_rs::auth::auth_loader::load_auth_from_file;
use kalshi_rs::KalshiClient;

pub fn setup_client() -> KalshiClient {
    let account = load_auth_from_file().expect("Failed to load auth credentials");
    KalshiClient::new(account)
}
