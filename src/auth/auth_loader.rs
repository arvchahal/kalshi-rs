use crate::auth::models::Account;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use rand::thread_rng;
use rsa::pss::SigningKey;
use rsa::signature::{RandomizedSigner, SignatureEncoding};
use rsa::{RsaPrivateKey, pkcs1::DecodeRsaPrivateKey, pkcs8::DecodePrivateKey};
use sha2::Sha256;
use std::env;
use std::fs;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};


// Environment variable names for authentication
const KALSHI_PK_FILE_PATH: &str = "KALSHI_PK_FILE_PATH";
const KALSHI_API_KEY_ID: &str = "KALSHI_API_KEY_ID";


/// Load authentication credentials from environment variables and file
///
/// Expects:
/// - KALSHI_API_KEY_ID: Your API key ID from Kalshi
/// - KALSHI_PK_FILE_PATH: Path to your private key PEM file
///
/// Returns an Account struct with credentials loaded
pub fn load_auth_from_file() -> io::Result<Account> {
    // Load API key ID from environment
    let api_key_id = env::var(KALSHI_API_KEY_ID)
        .map_err(|_| {
            eprintln!("{} is not set. Exiting.", KALSHI_API_KEY_ID);
            io::Error::new(
                io::ErrorKind::NotFound,
                "KALSHI_API_KEY_ID environment variable not set",
            )
        })?;

    // Load private key file path from environment
    let pk_file_path = env::var(KALSHI_PK_FILE_PATH)
        .map_err(|_| {
            eprintln!("{} is not set. Exiting.", KALSHI_PK_FILE_PATH);
            io::Error::new(
                io::ErrorKind::NotFound,
                "KALSHI_PK_FILE_PATH environment variable not set... please set an env variable to your Private key file path location (global or relative path works to run these tests)",
            )
        })?;

    // Read the private key PEM file
    // Handle the error gracefully since the file path comes from env var
    let private_key_pem = fs::read_to_string(&pk_file_path)
        .map_err(|e| {
            eprintln!("error {}", e);
            io::Error::new(io::ErrorKind::NotFound, "new weird error reading from file")
        })?;

    println!("Loaded private key from {}", &pk_file_path);
    Ok(Account::new(private_key_pem, api_key_id))
}


/// Get current timestamp in milliseconds
/// Used for request signing - TODO might delete if only used once
pub fn get_current_timestamp_ms() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("time should go forward");
    let in_ms = since_the_epoch.as_millis();
    in_ms.to_string()
}


/// Sign a request using RSA-PSS with SHA256
///
/// Kalshi requires signing: timestamp + method + path
/// (honestly weird choice by Kalshi but whatever)
pub fn sign_request(
    private_key_pem: &str,
    method: &str,
    path: &str,
    timestamp: u64,
) -> Result<String, Box<dyn std::error::Error>> {
    // Format the message string as required by Kalshi API
    let msg_string = format!("{}{}{}", timestamp, method, path);

    // Parse the private key - support both PKCS1 and PKCS8 formats
    // Kalshi allows both PKCS1 and PKCS8 PEM formats, so we try both
    let private_key = if private_key_pem.contains("BEGIN RSA PRIVATE KEY") {
        RsaPrivateKey::from_pkcs1_pem(private_key_pem)?
    } else {
        RsaPrivateKey::from_pkcs8_pem(private_key_pem)?
    };

    // Create signing key with PSS padding and SHA256
    // Kalshi API requires this specific signature format for auth
    let signing_key = SigningKey::<Sha256>::new_with_salt_len(private_key, 32);
    let mut rng = thread_rng();

    // Sign the message and encode to base64
    let signature = signing_key.sign_with_rng(&mut rng, msg_string.as_bytes());
    let sig_b64 = BASE64.encode(signature.to_bytes());

    Ok(sig_b64)
}
