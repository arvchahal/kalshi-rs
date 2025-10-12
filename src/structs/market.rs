use derive_more::Display;

pub mod models {
    use derive_more::Display;
    
    pub struct Market {}

    #[derive(Debug, Clone, Display)]
    #[display("Account: username={}, api_key={}", username, api_key)]
    pub struct Account {
        pub username: String,
        pub password: String,
        pub api_key: String
    }

    pub struct OrderRequest {}

    pub struct Position {
        user: Account
    }

    pub struct OrderStatus{

    }
}