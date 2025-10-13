use std::fs;
use std::io;
use std::env;
use crate::auth::models::Account;
use std::time::{SystemTime, UNIX_EPOCH};
use rsa::{RsaPrivateKey, pkcs8::DecodePrivateKey};
use rsa::pss::SigningKey;
use rsa::signature::{RandomizedSigner, SignatureEncoding};
use sha2::Sha256;
use rand::thread_rng;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};


const PRIVATE_KEY_FILE: &str = "kalshi_private.pem";
const KALSHI_API_KEY_ID: &str = "KALSHI_API_KEY_ID";

pub fn load_auth_from_file() -> io::Result<Account> {
    println!("{}", env::current_dir()?.display());

    // Get API key ID from environment variable
    let api_key_id = env::var(KALSHI_API_KEY_ID)
        .map_err(|_| {
            eprintln!("{} is not set. Exiting.", KALSHI_API_KEY_ID);
            io::Error::new(io::ErrorKind::NotFound, "KALSHI_API_KEY_ID environment variable not set")
        })?;

    println!("{} is set.", api_key_id);

    // Read the private key PEM file directly
    // This file should contain the RSA private key in PEM format:
    // -----BEGIN RSA PRIVATE KEY----- or -----BEGIN PRIVATE KEY-----
    let private_key_pem = fs::read_to_string(PRIVATE_KEY_FILE)?;

    println!("Loaded private key from {}", PRIVATE_KEY_FILE);

    Ok(Account::new(private_key_pem, api_key_id))
}

//simple helper for getting MS timestamp for methods requiring auth
pub fn get_current_timestamp_ms()->String{
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("time should go forward");
    let in_ms = since_the_epoch.as_millis();
    in_ms.to_string()

}

//
pub fn sign_request(
    private_key_pem: &str,
    method: &str,
    path: &str,
    timestamp: u64,
) -> Result<String, Box<dyn std::error::Error>> {
    let msg_string = format!("{}{}{}", timestamp, method, path);
    
    let private_key = RsaPrivateKey::from_pkcs8_pem(private_key_pem)?;
    
    let signing_key = SigningKey::<Sha256>::new(private_key);
    
    let mut rng = thread_rng();

    let signature = signing_key.sign_with_rng(&mut rng, msg_string.as_bytes());
    
    // Encode to base64
    Ok(BASE64.encode(signature.to_bytes()))
}

