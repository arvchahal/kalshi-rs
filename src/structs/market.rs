pub mod models {
    pub struct Market {}

    struct Account {
        username: String,
        password: String,
        api_key: String
    }

    pub struct OrderRequest {}

    pub struct Position {
        user: Account
    }

    pub struct OrderStatus{

    }
}