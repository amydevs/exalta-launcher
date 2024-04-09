use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Url,
};

pub mod auth;
pub mod download;
pub mod misc;

static BASE_URL_STRING: Lazy<Url> =
    Lazy::new(|| Url::parse("https://www.realmofthemadgod.com/").unwrap());
static TESTING_BASE_URL_STRING: Lazy<Url> =
    Lazy::new(|| Url::parse("https://test.realmofthemadgod.com/").unwrap());

pub static BUILD_TYPE: Lazy<RwLock<Build>> = Lazy::new(|| RwLock::new(Build::Production));
static CLIENT_TOKEN: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::new()));

pub static DEFAULT_PARAMS: Lazy<RwLock<Vec<(String, String)>>> = Lazy::new(|| {
    RwLock::new(vec![
        (String::from("game_net"), String::from("Unity")),
        (String::from("play_platform"), String::from("Unity")),
        (String::from("game_net_user_id"), String::from("")),
    ])
});
static CLIENT: Lazy<Client> = Lazy::new(|| {
    let mut defheaders = HeaderMap::new();
    defheaders.insert("Accept", "*/*".parse().unwrap());
    defheaders.insert("X-Unity-Version", HeaderValue::from_static("2020.3.30f1"));
    Client::builder()
        .http1_title_case_headers()
        .user_agent("UnityPlayer/2020.3.30f1 (UnityWebRequest/1.0, libcurl/7.80.0-DEV)")
        .default_headers(defheaders)
        .build()
        .unwrap()
});

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Build {
    Production,
    Testing,
}
impl std::fmt::Display for Build {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub async fn get_base_url() -> &'static Url {
    get_base_url_from_build_type(&*BUILD_TYPE.read().await)
}
pub fn get_base_url_force() -> &'static Url {
    get_base_url_from_build_type(&*BUILD_TYPE.try_read().unwrap())
}
fn get_base_url_from_build_type(build_type: &Build) -> &'static Url {
    return match *build_type {
        Build::Production => &BASE_URL_STRING,
        Build::Testing => &TESTING_BASE_URL_STRING,
    };
}

pub async fn set_build(build: Build) {
    *BUILD_TYPE.write().await = build;
}
pub fn set_build_force(build: Build) {
    *BUILD_TYPE.try_write().unwrap() = build;
}

pub fn set_steamid_game_net_play_platform(steamid: &str) {
    let s = "Unity_steam".to_string();
    let params = &mut DEFAULT_PARAMS.blocking_write();
    for (key, val) in params.iter_mut() {
        match key.as_str() {
            "game_net" | "play_platform" => *val = s.clone(),
            "game_net_user_id" => *val = steamid.to_owned(),
            _ => {}
        }
    }
    params.push(("steamid".to_owned(), steamid.to_owned()));
}

pub fn set_client_token(token: &str) {
    *CLIENT_TOKEN.blocking_write() = token.to_owned();
}

pub fn coll_to_owned(vec: Vec<(&str, &str)>) -> Vec<(String, String)> {
    vec.iter()
        .map(|e| (e.0.to_owned(), e.1.to_owned()))
        .collect()
}
