use std::str::FromStr;

use reqwest::Client;
use crate::markets::models::Account;

const KALSHI_API: &str = "https://api.elections.kalshi.com/trade-api/v2";

struct KalshiClient{
    http_client: Client,
    account: Account,
    base_url: String
}

impl KalshiClient{
    pub fn new(user:Account) -> KalshiClient{
        KalshiClient{
            http_client: Client::new(),
            account:user,
            base_url: KALSHI_API.to_string(),
        }
    }
}