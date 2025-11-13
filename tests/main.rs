use kalshi_rust_sdk::auth::auth_loader::load_auth_from_file;
use kalshi_rust_sdk::{Account, KalshiClient};
fn main() {
    let account = load_auth_from_file().unwrap();
    let client = KalshiClient::new(account);
}
