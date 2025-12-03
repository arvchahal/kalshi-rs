# Contributing

Thanks for your interest in contributing to **kalshi-rs**! First things first

## Setup

1. **Clone the repo:**

```bash
git clone https://github.com/arnavchahal/kalshi-rs.git
cd kalshi-rs
```

2. **Export your Kalshi auth variables:**

```bash
export KALSHI_PK_FILE_PATH="/path/to/kalshi_private.pem"
export KALSHI_API_KEY_ID="your_api_key_id"
```

These correspond to:

```rust
const KALSHI_PK_FILE_PATH: &str = "KALSHI_PK_FILE_PATH";
const KALSHI_API_KEY_ID: &str = "KALSHI_API_KEY_ID";
```

## Testing

Run the full test suite to confirm all endpoints work:

```bash
cargo test
```

If you add new endpoints or features, **add tests** that cover the new behavior.

## Guidelines

* Follow the existing layout in `src/` and the patterns described in the README.
* Match the style and structure of existing modules (auth, markets, trading, portfolio, etc.).
* Keep responses strongly typed and avoid untyped JSON unless necessary.

That's it — thank you for improving the SDK!
