use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Credential {
    pub site: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Vault {
    pub credentials: Vec<Credential>,
}