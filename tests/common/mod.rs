use kalshi_rs::KalshiClient;
use kalshi_rs::KalshiWebsocketClient;
use kalshi_rs::auth::auth_loader::load_auth_from_file;


pub fn setup_client() -> KalshiClient {
    let account = load_auth_from_file().expect("Failed to load auth credentials");
    KalshiClient::new(account)
}

pub fn setup_ws_client() -> KalshiWebsocketClient {
    let account = load_auth_from_file().expect("Failed to load auth credentials");
    KalshiWebsocketClient::new(account)
}
