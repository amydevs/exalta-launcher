use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LaunchArgs {
    pub platform: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steam_id: Option<String>,
    pub guid: String,
    pub token: String,
    pub token_timestamp: String,
    pub token_expiration: String,
    pub env: i32,
    pub server_name: String,
}
