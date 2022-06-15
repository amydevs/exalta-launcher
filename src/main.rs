use clap::Parser;
use directories::UserDirs;
use exalta_core::ExaltaClient;
use launchargs::LaunchArgs;
use tokio::process::Command;

mod args;
mod launchargs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //init args
    let args = crate::args::Args::parse();
    let USERNAME = args.username.as_str();
    let PASSWORD = args.password.as_str();

    let exalta_client = ExaltaClient::new()?;
    let authcon = exalta_client.login(USERNAME, PASSWORD).await?;
    authcon.verify().await?;
    
    if let Some(user_dirs) = UserDirs::new() {
        if let Some(document_dir) = user_dirs.document_dir() {
            let execpath = document_dir.join("RealmOfTheMadGod/Production/RotMG Exalt.exe");
            let args = serde_json::to_string(&LaunchArgs {
                platform: "Deca".to_string(),
                guid: base64::encode(USERNAME),
                token: base64::encode(authcon.account.access_token),
                token_timestamp: base64::encode(authcon.account.access_token_timestamp),
                token_expiration: base64::encode(authcon.account.access_token_expiration.clone()),
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