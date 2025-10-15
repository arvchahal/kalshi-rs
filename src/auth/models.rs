
#[derive(Debug, Clone)]
pub struct Account {
    private_key_pem: String,
    key_id: String
}

impl Account {
    pub fn new( private_key_pem: String,key_id:String) -> Self {
        Self {
            private_key_pem,
            key_id,
        }
    }



    pub fn private_key_pem(&self) -> &str {
        &self.private_key_pem
    }

    pub fn key_id(&self)->&str{
        &self.key_id
    }
}
