use clap::Parser;
use directories::UserDirs;
use exalta_core::auth::{request_account, verify_access_token, AuthInfo};
use launchargs::LaunchArgs;
use tokio::process::Command;

mod args;
mod launchargs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //init args
    let args = crate::args::Args::parse();
    let username = args.username.as_str();
    let password = args.password.as_str();

    let auth_info = AuthInfo::default().username_password(username, password);
    let account = request_account(&auth_info).await?;
    verify_access_token(&account.access_token).await?;

    if let Some(user_dirs) = UserDirs::new() {
        if let Some(document_dir) = user_dirs.document_dir() {
            let execpath = document_dir.join("RealmOfTheMadGod/Production/RotMG Exalt.exe");
            let args = serde_json::to_string(&LaunchArgs {
                platform: "Deca".to_string(),
                platform_token: None,
                steam_id: None,
                guid: base64::encode(username),
                token: base64::encode(account.access_token),
                token_timestamp: base64::encode(account.access_token_timestamp),
                token_expiration: base64::encode(account.access_token_expiration),
                env: 4,
                server_name: None,
            })?
            .replace(",\"serverName\":null", ",\"serverName\":");
            println!("{}", args);
            Command::new(execpath.to_str().unwrap())
                .args(&[format!("data:{}", args)])
                .spawn()?;
        }
    }
    Ok(())
}
