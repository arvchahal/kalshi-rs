Got you — here is a **clean, professional, non-cringe, non-fluffy** README modeled in the style of the prettyplease repo, but rewritten for your `kalshi-rs` SDK exactly the way you asked:

* **No feature matrix**
* **Clear shout-out that the API design strictly mirrors the official Python SDK**
* **No goofy “design notes” — only practical information**
* **MIT license section**
* **Contributing section welcoming PRs**
* **Cargo test note + link to Kalshi docs for API key setup**
* **Same badge style + formatting style from prettyplease**

Here we go:

---

# kalshi-rs

=====================

[<img alt="github" src="https://img.shields.io/badge/github-arnavchahal/kalshi--rs-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/arnavchahal/kalshi-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/kalshi-rs.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/kalshi-rs)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-kalshi--rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/kalshi-rs)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/arnavchahal/kalshi-rs/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/arnavchahal/kalshi-rs/actions?query=branch%3Amain)

A fast, strongly-typed Rust SDK for the **Kalshi exchange API**.

<br>

## Overview

`kalshi-rs` is a Rust client that mirrors the structure and behavior of the
official **Kalshi Python SDK**
➡ [https://docs.kalshi.com/sdks/python/quickstart](https://docs.kalshi.com/sdks/python/quickstart)

This means:

* method names match the Python SDK whenever possible
* request/response structures follow the same shape
* authentication and request signing work *exactly* like the official version
* examples from the Python docs can be translated to Rust easily

If you’ve used Kalshi’s Python SDK, the Rust version will feel immediately familiar.

The crate supports:

* authentication using your Kalshi API key & PEM private key
* listing and querying markets
* retrieving orderbooks, trades, candlesticks
* creating, canceling, and managing orders
* fetching portfolio and positions
* error handling consistent with Kalshi’s API responses

<br>

## Quickstart Example

```rust
use kalshi_rs::{KalshiClient, Account};
use kalshi_rs::markets::models::MarketsQuery;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key_id = std::env::var("KALSHI_API_KEY_ID")?;
    let account = Account::from_file("kalshi_private.pem", api_key_id)?;
    let client = KalshiClient::new(account);

    let params = MarketsQuery {
        status: Some("active".into()),
        limit: Some(10),
        ..Default::default()
    };

    let markets = client.get_all_markets(&params).await?;
    for m in markets.markets {
        println!("{} — {}", m.ticker, m.title);
    }

    Ok(())
}
```

<br>

## Placing and Canceling an Order

```rust
use kalshi_rs::portfolio::models::CreateOrderRequest;

let order = CreateOrderRequest {
    ticker: "KXWTAMATCH".into(),
    action: "buy".into(),
    side: "yes".into(),
    count: 1,
    type_: Some("limit".into()),
    yes_price: Some(3),
    ..Default::default()
};

let placed = client.create_order(&order).await?;
println!("Order ID: {}", placed.order.order_id);

client.cancel_order(placed.order.order_id.clone()).await?;
println!("Order canceled.");
```

<br>

## Running Tests

Several tests interact with the real Kalshi API.
You **must** set your environment variables:

```bash
export KALSHI_API_KEY_ID="your_key_id"
export KALSHI_PRIVATE_KEY_PATH="path/to/your/kalshi_private.pem"
```

You can generate these keys from the Kalshi dashboard:

➡ **API Keys Setup:** [https://docs.kalshi.com/introduction/#authentication](https://docs.kalshi.com/introduction/#authentication)

Then run:

```bash
cargo test -- --nocapture
```

Tests involving trading endpoints will make **real requests**, so be mindful of costs.

<br>

## Design Philosophy

`kalshi-rs` intentionally follows the **official Python SDK’s structure and naming** to make the transition between languages seamless. If the Python docs say:

```python
client.markets.get_markets(...)
```

You can expect the Rust version to expose an equivalent:

```rust
client.get_all_markets(...)
```

The goal is clarity, reliability, and easy adoption — not reinvention.

<br>

## Contributing

Contributions are welcome!

* create a branch
* add your features / improvements
* open a pull request

If you're implementing newly released Kalshi endpoints, improving error handling, or adding examples, all PRs are appreciated.

<br>

## License

This project is licensed under the **MIT License**.
See [`LICENSE`](LICENSE-MIT) for details.


