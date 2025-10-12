// use derive_more::Display;

    use derive_more::Display;
    
pub struct Market {}

    #[derive(Debug, Clone, Display)]
    #[display("Account: username={}, api_key={}", username, api_key)]
    pub struct Account {
        username: String,
        password: String,
        api_key: String
    }
    impl Account {
        pub fn new(username: String, password: String, api_key: String) -> Self {
            Self {
                username,
                password,
                api_key,
            }
        }

        // Getters
        pub fn username(&self) -> &str {
            &self.username
        }

        pub fn password(&self) -> &str {
            &self.password
        }

        pub fn api_key(&self) -> &str {
            &self.api_key
        }
    }

    pub struct OrderRequest {}

    pub struct Position {
        user: Account
    }

    pub struct OrderStatus{

    }
