use exalta_core::auth::steamworks::Credentials;

pub struct SteamWrapper {
    steam_client: Option<(::steamworks::Client, ::steamworks::SingleClient)>,
    steam_credentials: Option<Credentials>,
}

impl Default for SteamWrapper {
    fn default() -> Self {
        Self {
            steam_client: ::steamworks::Client::init_app(200210).ok(),
            steam_credentials: None,
        }
    }
}


pub trait SteamTrait {
    fn get_client(&self) -> Option<&(::steamworks::Client, ::steamworks::SingleClient)>;
    fn get_credentials(&self) -> Option<&Credentials>;
}

impl SteamTrait for SteamWrapper {
    fn get_client(&self) -> Option<&(::steamworks::Client, ::steamworks::SingleClient)> {
        self.steam_client.as_ref()
    }

    fn get_credentials(&self) -> Option<&Credentials> {
        self.steam_credentials.as_ref()
    }
}


impl SteamTrait for Option<Box<dyn SteamTrait>> {
    fn get_client(&self) -> Option<&(::steamworks::Client, ::steamworks::SingleClient)> {
        None
    }

    fn get_credentials(&self) -> Option<&Credentials> {
        None
    }
}