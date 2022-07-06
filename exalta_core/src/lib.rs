use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Url,
};

pub mod auth;
pub mod download;
pub mod misc;

static BASE_URL: Lazy<Url> = Lazy::new(|| Url::parse("https://www.realmofthemadgod.com/").unwrap());
static CLIENT_TOKEN: Lazy<RwLock<&str>> = Lazy::new(|| RwLock::new(""));

pub static DEFAULT_PARAMS: Lazy<RwLock<Vec<(String, String)>>> = Lazy::new(|| {
    RwLock::new(vec![
        (String::from("game_net"), String::from("Unity")),
        (String::from("play_platform"), String::from("Unity")),
        (String::from("game_net_user_id"), String::from("")),
    ])
});
static CLIENT: Lazy<Client> = Lazy::new(|| {
    let mut defheaders = HeaderMap::new();
    defheaders.insert("Host", BASE_URL.host_str().unwrap().parse().unwrap());
    defheaders.insert("Accept", "*/*".parse().unwrap());
    defheaders.insert("Accept-Encoding", HeaderValue::from_static("gzip, deflate"));
    defheaders.insert("X-Unity-Version", HeaderValue::from_static("2020.3.30f1"));
    Client::builder()
        .http1_title_case_headers()
        .user_agent("UnityPlayer/2020.3.30f1 (UnityWebRequest/1.0, libcurl/7.80.0-DEV)")
        .default_headers(defheaders)
        .build()
        .unwrap()
});

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

pub fn set_client_token(token: &'static str) {
    *CLIENT_TOKEN.blocking_write() = token;
}

pub fn coll_to_owned(vec: Vec<(&str, &str)>) -> Vec<(String, String)> {
    vec.iter()
        .map(|e| (e.0.to_owned(), e.1.to_owned()))
        .collect()
}
