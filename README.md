[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/A2XMx1Wt)
# kalshi-rust-sdk

## Description

A Rust SDK for interacting with the Kalshi trading API. Kalshi is an event-based prediction market platform where users can trade on the outcomes of real-world events. This project provides a Rust interface to Kalshi's trading API, enabling developers to build trading applications and market analysis tools. The SDK handles authentication, market data retrieval, API key management, and exchange information queries.

Third-party crates used: **reqwest** (HTTP client), **tokio** (async runtime), **serde/serde_json** (JSON serialization), **rsa/sha2/base64** (RSA cryptography for request signing), **url** (URL parsing), and **derive_more** (derive macros).

## Installation

**Prerequisites:** Rust installed via rustup, a Kalshi account, and API credentials.

**Setup:**
1. Clone the repository and navigate to the project directory
2. Create a file named `kalshi_private.txt` in the project root containing your RSA private key in PEM format
3. Set your Kalshi API key ID as an environment variable
4. Compile and install:
```bash
cargo build --release
```

## How to use

After compilation and setup, run the main binary to interact with the Kalshi API:
```bash
cargo run
```

The SDK provides methods for API key management (list, generate, delete), retrieving exchange information (status, announcements, schedule), and accessing market data.
