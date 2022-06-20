use once_cell::sync::Lazy;
use std::sync::{Mutex, RwLock};

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Url,
};

pub mod auth;
pub mod misc;

const BASE_URL: Lazy<Url> = Lazy::new(|| {
    Url::parse("https://www.realmofthemadgod.com/").unwrap()
});
const CLIENT_TOKEN: &str = "6f97fc3698b237db27591d6b431a9532b14d1922";

static DEFAULT_PARAMS: Lazy<RwLock<Vec<(String, String)>>> = Lazy::new(|| {
    RwLock::new(vec![
        (String::from("game_net"), String::from("Unity")),
        (String::from("play_platform"), String::from("Unity")),
        (String::from("game_net_user_id"), String::from("")),
    ])
});
const CLIENT: Lazy<Client> = Lazy::new(|| {
    let mut defheaders = HeaderMap::new();
    defheaders.insert("Host", BASE_URL.host_str().unwrap().parse().unwrap());
    defheaders.insert("Accept", "*/*".parse().unwrap());
    defheaders.insert("Accept-Encoding", HeaderValue::from_static("gzip, deflate"));
    defheaders.insert("X-Unity-Version", HeaderValue::from_static("2020.3.30f1"));
    Client::builder()
        .http1_title_case_headers()
        .user_agent("UnityPlayer/2020.3.30f1 (UnityWebRequest/1.0, libcurl/7.80.0-DEV)")
        .default_headers(defheaders)
        .build().unwrap()
});

pub fn set_game_net_play_platform(game_net: &str) {
    let s = game_net.to_owned();
    DEFAULT_PARAMS.write().unwrap()[0].1 = s.clone();
    DEFAULT_PARAMS.write().unwrap()[1].1 = s;
}

pub fn coll_to_owned(vec: Vec<(&str, &str)>) -> Vec<(String, String)> {
    vec.iter().map(|e| (e.0.to_owned(), e.1.to_owned())).collect()
}