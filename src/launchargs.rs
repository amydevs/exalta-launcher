use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LaunchArgs {
    pub platform: String,
    pub platform_token: Option<String>,
    pub steam_id: Option<String>,
    pub guid: String,
    pub token: String,
    pub token_timestamp: String,
    pub token_expiration: String,
    pub env: i32,
    pub server_name: Option<String>,
}
