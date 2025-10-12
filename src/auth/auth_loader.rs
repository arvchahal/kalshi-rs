use std::fs;
use std::io;
use structs::username;
mod auth{
    const SECRETS_FILE_DEFAULT: &str = "kalshi_private.txt";

    fn load_auth_from_file() -> io::Result<(String, String, String)> {
        let contents = fs::read_to_string(file_path)?;
        println!("File contents:\n{}", contents);
        Ok(())
    }

    fn load_from_input(username: &str, password:&str, api_key:&str){

    }
}