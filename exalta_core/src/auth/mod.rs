use crate::{coll_to_owned, get_base_url, CLIENT, DEFAULT_PARAMS};
use anyhow::Result;

use self::account::Account;
use self::err::AuthError;
use self::steamworks::Credentials;

pub mod account;
pub mod err;
pub mod steamworks;

pub struct AuthInfo {
    pub username: String,
    pub password: String,
    pub steamworks_credentials: Option<Credentials>,
}
impl Default for AuthInfo {
    fn default() -> Self {
        Self {
            username: Default::default(),
            password: Default::default(),
            steamworks_credentials: None,
        }
    }
}
impl AuthInfo {
    pub fn username_password(mut self, username: &str, password: &str) -> Self {
        self.username = username.to_string();
        self.password = password.to_string();
        self
    }
    pub fn steamworks_credentials(mut self, steamworks_credentials: Credentials) -> Self {
        self.steamworks_credentials = Some(steamworks_credentials);
        self
    }
}
pub async fn request_account(auth_info: &AuthInfo) -> Result<Account> {
    let tokenparams = coll_to_owned(vec![("clientToken", &crate::CLIENT_TOKEN.read().await)]);
    let post_params: Result<Vec<(String, String)>> =
        if !auth_info.password.is_empty() && !auth_info.username.is_empty() {
            Ok([
                tokenparams,
                DEFAULT_PARAMS.read().await.to_vec(),
                coll_to_owned(vec![
                    ("guid", &auth_info.username),
                    ("password", &auth_info.password),
                ]),
            ]
            .concat())
        } else if let Some(steam_creds) = &auth_info.steamworks_credentials {
            Ok([
                coll_to_owned(vec![
                    ("guid", &steam_creds.guid),
                    ("secret", &steam_creds.secret),
                ]),
                tokenparams,
                DEFAULT_PARAMS.read().await.to_vec(),
            ]
            .concat())
        } else {
            return Err(AuthError(String::from("No Credentials")).into());
        };

    let resp = CLIENT
        .post(get_base_url().await.join("account/verify")?)
        .form(&post_params?)
        .send()
        .await?;

    let resp_text = resp.text().await?;

    if resp_text.to_lowercase().starts_with("<error>") {
        return Err(AuthError(String::from("Credentials Incorrect")).into());
    }
    Ok(quick_xml::de::from_str::<Account>(resp_text.as_str())
        .map_err(|e| AuthError(e.to_string()))?)
}

pub async fn request_forgot_password(guid: &str) -> Result<()> {
    let params = [
        coll_to_owned(vec![("guid", guid)]),
        DEFAULT_PARAMS.read().await.to_vec(),
    ]
    .concat();
    let resp = CLIENT
        .post(get_base_url().await.join("account/forgotPassword")?)
        .form(&params)
        .send()
        .await?;

    let resp_text = resp.text().await?;
    if !resp_text.to_lowercase().contains("success") {
        let error_less_text = resp_text.replace("<Error>", "").replace("</Error>", "");
        return Err(AuthError(error_less_text).into());
    }
    Ok(())
}

pub async fn verify_access_token(access_token: &str) -> Result<bool> {
    // verify
    let tokenparams = coll_to_owned(vec![
        ("clientToken", &crate::CLIENT_TOKEN.read().await),
        ("accessToken", access_token),
    ]);
    let userpassparams = [tokenparams, crate::DEFAULT_PARAMS.read().await.to_vec()].concat();
    let resp = CLIENT
        .post(
            get_base_url()
                .await
                .join("account/verifyAccessTokenClient")?,
        )
        .form(&userpassparams)
        .send()
        .await?;
    let boolcheck = resp.text().await?.to_lowercase().contains("success");
    if boolcheck {
        Ok(boolcheck)
    } else {
        return Err(AuthError(String::from("Invalid access token!")).into());
    }
}
