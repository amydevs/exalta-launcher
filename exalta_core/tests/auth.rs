#[cfg(test)]
mod tests {
    use exalta_core::auth::{account::Account, AuthInfo};

    #[tokio::test]
    async fn test_login() {
        exalta_core::auth::request_account(&get_env_auth_info()).await.unwrap();
    }

    #[tokio::test]
    async fn test_verify() {
        let account = exalta_core::auth::request_account(&get_env_auth_info()).await.unwrap();
        exalta_core::auth::verify_access_token(&account.access_token).await.unwrap();
    }

    fn get_env_auth_info() -> AuthInfo {
        dotenv::from_path("./tests/.env").unwrap();
        exalta_core::auth::AuthInfo::default().username_password(
            &std::env::var("USERNAME").unwrap(),
            &std::env::var("PASSWORD").unwrap()
        )
    }

    #[tokio::test]
    async fn test_set_steam() {
        exalta_core::set_steamid_game_net_play_platform("sdsdkj");
        println!("{:?}", exalta_core::DEFAULT_PARAMS.read().unwrap());
    }
}
