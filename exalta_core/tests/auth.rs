#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_login() {
        let (u, p) = get_env_up();
        let client = exalta_core::auth::AuthInfo::default().username_password(&u, &p);
        exalta_core::auth::request_account(&client).await.unwrap();
    }

    #[tokio::test]
    async fn test_verify() {
        let (u, p) = get_env_up();
        let client = exalta_core::auth::AuthInfo::default().username_password(&u, &p);
        let account = exalta_core::auth::request_account(&client).await.unwrap();
        exalta_core::auth::verify_access_token(&account.access_token).await.unwrap();
    }

    fn get_env_up() -> (String, String) {
        dotenv::from_path("./tests/.env").unwrap();
        (std::env::var("USERNAME").unwrap(), std::env::var("PASSWORD").unwrap())
    }
}
