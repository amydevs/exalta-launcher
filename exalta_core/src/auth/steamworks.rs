use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Credentials {
    #[serde(rename = "GUID")]
    pub guid: String,

    pub secret: String,

    pub platform_token: String,

    pub name: String,

    pub name_chosen: String,

    pub access_token: String,

    pub access_token_timestamp: String,

    pub access_token_expiration: String,
}
