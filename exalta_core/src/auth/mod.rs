use reqwest::{Client, Url};


use crate::{CLIENT_TOKEN, DEFAULT_PARAMS, BASE_URL, CLIENT, coll_to_owned};

use self::account::Account;
use self::err::AuthError;

pub mod account;
pub mod err;

pub struct AuthInfo {
    pub username: String,
    pub password: String,
    pub session_token: String
}
impl Default for AuthInfo {
    fn default() -> Self {
        Self { username: Default::default(), password: Default::default(), session_token: Default::default() }
    }
}
impl AuthInfo {
    pub fn username_password(mut self, username: &str, password: &str) -> Self {
        self.username = username.to_string();
        self.password = password.to_string();
        self
    }
    pub fn session_token(mut self, session_token: &str) -> Self {
        self.session_token = session_token.to_string();
        self
    }
}
pub async fn request_account(
    auth_info: &AuthInfo
) -> Result<Account, Box<dyn std::error::Error>> {
    if !auth_info.password.is_empty() && !auth_info.username.is_empty() {

        let tokenparams = coll_to_owned(vec![("clientToken", CLIENT_TOKEN)]);

        let userpassparams = [
            tokenparams,
            DEFAULT_PARAMS.read()?.to_vec(),
            coll_to_owned(vec![("guid", &auth_info.username), ("password", &auth_info.password)]),
        ]
        .concat();
        let resp = CLIENT
            .post(BASE_URL.join("account/verify")?)
            .form(&userpassparams)
            .send()
            .await?;

        let resp_text = resp.text().await?;
        if resp_text.to_lowercase().starts_with("<error>") {
            return Err(AuthError(String::from("Credentials Incorrect")).into());
        }
        Ok(quick_xml::de::from_str::<Account>(resp_text.as_str())
            .map_err(|e| AuthError(e.to_string()))?)
    }
    else {
        todo!()
    }
}

pub async fn verify_access_token(access_token: &str) -> Result<bool, Box<dyn std::error::Error>> {
    // verify
    let tokenparams = coll_to_owned(vec![
        ("clientToken", crate::CLIENT_TOKEN),
        ("accessToken", access_token),
    ]);
    let userpassparams = [tokenparams, crate::DEFAULT_PARAMS.read()?.to_vec()].concat();
    let resp = CLIENT
        .post(BASE_URL.join("account/verifyAccessTokenClient")?)
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