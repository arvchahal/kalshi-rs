use kalshi_rust_sdk::{Account, KalshiClient};
use kalshi_rust_sdk::auth::auth_loader::{load_auth_from_file};

fn main(){
    let account = load_auth_from_file().unwrap();
    let client = KalshiClient::new(account);
}