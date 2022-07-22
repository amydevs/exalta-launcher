use serde::{Deserialize, Serialize};

use crate::{coll_to_owned, get_base_url, CLIENT, DEFAULT_PARAMS};

use super::err::AuthError;

use anyhow::Result;

pub fn encode_hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

pub async fn request_credentials(session_token: &str) -> Result<Credentials> {
    let sessionticketparams = [
        coll_to_owned(vec![("sessionticket", session_token)]),
        DEFAULT_PARAMS.read().await.to_vec(),
    ]
    .concat();
    let steam_creds_resp = CLIENT
        .post(get_base_url().await.join("steamworks/getcredentials")?)
        .form(&sessionticketparams)
        .send()
        .await?;
    let resp_text = steam_creds_resp.text().await?;
    Ok(quick_xml::de::from_str::<Credentials>(&resp_text).map_err(|e| AuthError(e.to_string()))?)
}

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
