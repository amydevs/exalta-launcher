use reqwest::{Client, Url};

use self::account::Account;
use self::err::AuthError;

pub mod account;
pub mod err;

pub struct AuthController {
    pub client: Client,
    pub base_url: Url,

    pub account: Account,
}

impl AuthController {
    pub async fn verify(&self) -> Result<bool, Box<dyn std::error::Error>> {
        // verify
        let tokenparams = vec![
            ("clientToken", crate::CLIENT_TOKEN),
            ("accessToken", &self.account.access_token),
        ];
        let userpassparams = [tokenparams.clone(), crate::DEFAULT_PARAMS.to_vec()].concat();
        let resp = self
            .client
            .post(self.base_url.join("account/verifyAccessTokenClient")?)
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
}
