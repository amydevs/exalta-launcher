use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")] 
pub struct Account {
    pub access_token: String,
    pub access_token_timestamp: String,
    pub access_token_expiration: String
}