use directories::UserDirs;
use eframe::egui::Ui;
use tokio::process::Command;

use crate::{launchargs::LaunchArgs, ExaltaLauncher};

impl ExaltaLauncher {
    pub fn render_play(&mut self, ui: &mut Ui) -> Result<(), Box<dyn std::error::Error>> {
        ui.vertical_centered_justified(|ui| {
            ui.add_space(10.);
            ui.vertical_centered(|ui| {
                ui.label(format!(
                    "Welcome back, {}.\nYou have {} credits, {} forgefire, and {} fame.",
                    self.account.as_ref().unwrap().name,
                    self.account.as_ref().unwrap().credits,
                    self.account.as_ref().unwrap().forge_fire_energy,
                    self.account.as_ref().unwrap().stats.fame,
                ));
            });
            ui.add_space(10.);
            if ui.button("Play").clicked() {
                if self
                    .runtime
                    .block_on(exalta_core::auth::verify_access_token(
                        &self.account.as_ref().unwrap().access_token,
                    ))
                    .is_ok()
                {
                    self.load().ok();
                } else if self.login().is_err() {
                    self.account = None;
                }
            }
            ui.add_space(10.);
            if ui.button("Logout").clicked() {
                self.account = None;
            }
            Ok(())
        })
        .inner
    }
    fn load(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(user_dirs) = UserDirs::new() {
            if let Some(document_dir) = user_dirs.document_dir() {
                if let Some(account) = &self.account {
                    let execpath = document_dir.join("RealmOfTheMadGod/Production/RotMG Exalt.exe");
                    let args = if let Some(steam_creds) = &self.steam_credentials {
                        serde_json::to_string(&LaunchArgs {
                            platform: "Steam".to_string(),
                            guid: base64::encode(&self.auth.guid),
                            platform_token: Some(base64::encode(&steam_creds.platform_token)),
                            steam_id: Some(base64::encode(&self.auth.guid.replace("steamworks:", ""))),
                            token: base64::encode(account.access_token.clone()),
                            token_timestamp: base64::encode(account.access_token_timestamp.clone()),
                            token_expiration: base64::encode(account.access_token_expiration.clone()),
                            env: 4,
                            server_name: String::new(),
                        })?
                    }
                    else {
                        serde_json::to_string(&LaunchArgs {
                            platform: "Deca".to_string(),
                            guid: base64::encode(&self.auth.guid),
                            platform_token: None,
                            steam_id: None,
                            token: base64::encode(account.access_token.clone()),
                            token_timestamp: base64::encode(account.access_token_timestamp.clone()),
                            token_expiration: base64::encode(account.access_token_expiration.clone()),
                            env: 4,
                            server_name: String::new(),
                        })?
                    }.replace(",\"serverName\":null", ",\"serverName\":");
                    println!("{}", args);
                    Command::new(execpath.to_str().unwrap())
                        .args(&[format!("data:{}", args)])
                        .spawn()?;
                }
            }
        }
        Ok(())
    }
}
