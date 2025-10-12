use std::fs;
use std::io;
use std::env;
use crate::structs::market::models::Account;

const SECRETS_FILE_DEFAULT: &str = "kalshi_private.txt";

pub fn load_auth_from_file() -> io::Result<Account> {
    println!("{}",env::current_dir()?.display());

    let contents = fs::read_to_string(SECRETS_FILE_DEFAULT)?;
    println!("File contents:\n{}", contents);
    
    let parts: Vec<&str> = contents.trim().split(':').collect();
    if parts.len() != 3 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid file format. Expected username:password:api_key"
        ));
    }
    
    Ok(Account {
        username: parts[0].to_string(),
        password: parts[1].to_string(),
        api_key: parts[2].to_string(),
    })
}

pub fn load_from_input(username: &str, password: &str, api_key: &str) -> Account {
    Account {
        username: username.to_string(),
        password: password.to_string(),
        api_key: api_key.to_string(),
    }
}