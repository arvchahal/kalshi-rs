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
const KALSHI_PK_FILE_PATH: &str = "KALSHI_PK_FILE_PATH";
const KALSHI_API_KEY_ID: &str = "KALSHI_API_KEY_ID";
pub fn load_auth_from_file() -> io::Result<Account> {
    let api_key_id = env::var(KALSHI_API_KEY_ID)
        .map_err(|_| {
            eprintln!("{} is not set. Exiting.", KALSHI_API_KEY_ID);
            io::Error::new(
                io::ErrorKind::NotFound,
                "KALSHI_API_KEY_ID environment variable not set",
            )
        })?;
    let pk_file_path = env::var(KALSHI_PK_FILE_PATH)
        .map_err(|_| {
            eprintln!("{} is not set. Exiting.", KALSHI_PK_FILE_PATH);
            io::Error::new(
                io::ErrorKind::NotFound,
                "KALSHI_PK_FILE_PATH environment variable not set... please set an env variable to your Private key file path location (global or relative path works to run these tests)",
            )
        })?;
    let private_key_pem = fs::read_to_string(&pk_file_path)
        .map_err(|e| {
            eprintln!("error {}", e);
            io::Error::new(io::ErrorKind::NotFound, "new weird error reading from file")
        })?;
    println!("Loaded private key from {}", & pk_file_path);
    Ok(Account::new(private_key_pem, api_key_id))
}
pub fn get_current_timestamp_ms() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("time should go forward");
    let in_ms = since_the_epoch.as_millis();
    in_ms.to_string()
}
pub fn sign_request(
    private_key_pem: &str,
    method: &str,
    path: &str,
    timestamp: u64,
) -> Result<String, Box<dyn std::error::Error>> {
    let msg_string = format!("{}{}{}", timestamp, method, path);
    let private_key = if private_key_pem.contains("BEGIN RSA PRIVATE KEY") {
        RsaPrivateKey::from_pkcs1_pem(private_key_pem)?
    } else {
        RsaPrivateKey::from_pkcs8_pem(private_key_pem)?
    };
    let signing_key = SigningKey::<Sha256>::new_with_salt_len(private_key, 32);
    let mut rng = thread_rng();
    let signature = signing_key.sign_with_rng(&mut rng, msg_string.as_bytes());
    let sig_b64 = BASE64.encode(signature.to_bytes());
    Ok(sig_b64)
}
