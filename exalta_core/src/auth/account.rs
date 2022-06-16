use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")] 
pub struct Account {
    pub name: String,
    pub access_token: String,
    pub access_token_timestamp: String,
    pub access_token_expiration: String
}