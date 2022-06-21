#[cfg(test)]
mod tests {
    use exalta_core::auth::{account::Account, AuthInfo, steamworks::encode_hex};

    #[tokio::test]
    async fn test_login() {
        exalta_core::auth::request_account(&get_env_auth_info()).await.unwrap();
    }

    #[test]
    fn test_login_steam() {
        let runtime = ::tokio::runtime::Runtime::new().unwrap();
        let (client, single) = ::steamworks::Client::init_app(200210).unwrap();
        exalta_core::set_steamid_game_net_play_platform(&client.user().steam_id().raw().to_string());
        println!("{:?}", exalta_core::DEFAULT_PARAMS.read().unwrap());
        let user = client.user();

        let _cb = client
            .register_callback(move |v: ::steamworks::AuthSessionTicketResponse| {
                println!("Got response: {:?}", v.result);
                
            });
        let _cb = client.register_callback(|v: ::steamworks::ValidateAuthTicketResponse| println!("{:?}", v));

        let id = user.steam_id();
        let (auth, ticket) = user.authentication_session_ticket();
            
        println!("BEGIN {:?}", user.begin_authentication_session(id, &ticket));

        for _ in 0..20 {
            single.run_callbacks();
            ::std::thread::sleep(::std::time::Duration::from_millis(50));
        }
        
        println!("END");
        runtime.block_on(
            exalta_core::auth::request_account(&AuthInfo::default().session_token(&encode_hex(&ticket)))
        ).unwrap();

        user.cancel_authentication_ticket(auth);

        for _ in 0..20 {
            single.run_callbacks();
            ::std::thread::sleep(::std::time::Duration::from_millis(50));
        }

        user.end_authentication_session(id);
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

    #[test] 
    fn test_serde() {
        let string = "<Account></Account>";
        quick_xml::de::from_str::<Account>(string).unwrap();
    }
}
