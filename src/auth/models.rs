use derive_more::Display;

#[derive(Debug, Clone, Display)]
#[display("Account: username={}", username)]
pub struct Account {
    username: String,
    private_key_pem: String,
}

impl Account {
    pub fn new(username: String, private_key_pem: String) -> Self {
        Self {
            username,
            private_key_pem,
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn private_key_pem(&self) -> &str {
        &self.private_key_pem
    }
}
