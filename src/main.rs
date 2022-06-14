use std::collections::HashMap;

use account::Account;
use clap::Parser;
use directories::UserDirs;
use launchargs::LaunchArgs;
use reqwest::{Url, header::{HeaderMap, HeaderValue}};
use tokio::process::Command;

mod args;
mod account;
mod launchargs;

const BASE_URL: &str = "https://www.realmofthemadgod.com/";

const CLIENT_TOKEN: &str = "6f97fc3698b237db27591d6b431a9532b14d1922";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //init args
    let args = crate::args::Args::parse();
    let USERNAME = args.username.as_str();
    let PASSWORD = args.password.as_str();

    let base_url = Url::parse(BASE_URL)?;
    let domain = base_url.host_str().unwrap();

    // init
    let mut defheaders = HeaderMap::new();
    defheaders.insert("Host", domain.parse()?);
    defheaders.insert("Accept", "*/*".parse()?);
    defheaders.insert( "Accept-Encoding", HeaderValue::from_static("gzip, deflate"));
    defheaders.insert( "X-Unity-Version", HeaderValue::from_static("2020.3.30f1"));
    let client = reqwest::Client::builder()
        .http1_title_case_headers()
        .user_agent("UnityPlayer/2020.3.30f1 (UnityWebRequest/1.0, libcurl/7.80.0-DEV)")
        .default_headers(defheaders)
        .build()?;

    let mut tokenparams = vec![
        ("clientToken", CLIENT_TOKEN)
    ];
    let defparams = vec![
        ("game_net", "Unity"),
        ("play_platform", "Unity"),
        ("game_net_user_id", ""),
    ];

    // login
    let userpassparams = [
        tokenparams.clone(),
        defparams.clone(),
        vec![
            ("guid", USERNAME),
            ("password", PASSWORD),
        ]
    ].concat();
    let resp = client
        .post(base_url.join("account/verify")?)
        .form(&userpassparams)
        .send()
        .await?;
    let acc: Account = quick_xml::de::from_str(resp.text().await?.as_str())?;

    // verify
    tokenparams.push(("accessToken", &acc.access_token));
    let userpassparams = [
        tokenparams.clone(),
        defparams.clone()
    ].concat();
    let resp = client
        .post(base_url.join("account/verifyAccessTokenClient")?)
        .form(&userpassparams)
        .send()
        .await?;

    if resp.text().await?.to_lowercase().contains("success") {
        println!("verified");
    } else {
        println!("failed");
    };
    
    if let Some(user_dirs) = UserDirs::new() {
        if let Some(document_dir) = user_dirs.document_dir() {

            let execpath = document_dir.join("RealmOfTheMadGod/Production/RotMG Exalt.exe");
            let args = serde_json::to_string(&LaunchArgs {
                platform: "Deca".to_string(),
                guid: base64::encode(USERNAME),
                token: base64::encode(acc.access_token),
                token_timestamp: base64::encode(acc.access_token_timestamp),
                token_expiration: base64::encode(acc.access_token_expiration.clone()),
                env: 4,
                server_name: None,
            })?;
            println!("{}", args);
            Command::new(execpath.to_str().unwrap())
                .args(&[format!("data:{}", args)])
                .spawn()?;
        }
    }

    Ok(())
}