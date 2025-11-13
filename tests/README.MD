# Integration Tests Quick Reference

**Holds all integration tests for this SDK/Library**

## Setup Checklist

- [ ] Create `kalshi_private.pem` in project root with your RSA private key
- [ ] Set `KALSHI_API_KEY_ID` environment variable
- [ ] Verify: `echo $KALSHI_API_KEY_ID` shows your key
- [ ] Verify: `ls kalshi_private.pem` shows the file exists

## Quick Start

```bash
# 1. Set your API key ID
export KALSHI_API_KEY_ID="your-api-key-id-here"

# 2. Verify credentials work
cargo test test_get_balance -- --ignored --nocapture

# 3. Run all tests
cargo test -- --ignored --nocapture
```

## Test Files

| File | Description |
|------|-------------|
| `portfolio_integration_tests.rs` | Portfolio endpoints (balance, positions, fills, settlements, orders) |
| `markets_integration_tests.rs` | Market data (markets, history, orderbook, trades) |
| `events_integration_tests.rs` | Events (get events, filter by status/category) |
| `exchange_integration_tests.rs` | Exchange info (status, schedule, announcements) |
| `common/mod.rs` | Shared utilities for all tests |

## Common Commands

```bash
# Run specific test file
cargo test --test portfolio_integration_tests -- --ignored --nocapture

# Run single test
cargo test test_get_balance -- --ignored --nocapture

# Run tests matching pattern
cargo test positions -- --ignored --nocapture

# Run sequentially (avoid rate limits)
cargo test -- --ignored --nocapture --test-threads=1

# List all tests without running
cargo test -- --ignored --list
```

## API Environments

### Production (Default)
- URL: `https://api.elections.kalshi.com`
- **Uses real money!**
- Tests use `common::create_test_client()`

### Demo/Sandbox (Recommended)
- URL: `https://demo-api.kalshi.com`
- Paper trading only
- Change tests to use `common::create_demo_client()`

## Troubleshooting

| Error | Solution |
|-------|----------|
| `KALSHI_API_KEY_ID not set` | `export KALSHI_API_KEY_ID="your-key"` |
| `kalshi_private.pem not found` | Place PEM file in project root |
| `HTTP 401 Unauthorized` | Wrong API Key ID |
| `HTTP 403 Forbidden` | Wrong private key |

## Security

✅ **Safe to commit:**
- Test files
- This README
- Documentation

❌ **NEVER commit:**
- `kalshi_private.pem`
- Your API Key ID in code

See `../TESTING.md` for detailed setup instructions.