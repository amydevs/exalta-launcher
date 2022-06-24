use std::{ops::DerefMut, sync::Arc};

use directories::UserDirs;
use eframe::egui::{self, Ui};
use poll_promise::Promise;
use tokio::{process::Command, sync::RwLock};

use crate::{
    launchargs::LaunchArgs, main_ext::ResultTimeWrapper, update::UpdateError, ExaltaLauncher,
};

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
            let update_button = egui::Button::new("Update / Verify");
            if ui
                .add_enabled(self.download_finished_build_hash.is_none(), update_button)
                .clicked()
            {
                self.download();
            }

            if let Some(prom) = &self.download_finished_build_hash {
                match prom.ready() {
                    None => {}
                    Some(Err(_)) => {
                        self.run_res = ResultTimeWrapper::default();
                        self.run_res.result =
                            Err(Box::new(UpdateError("Download Failed!".to_string())));
                    }
                    Some(Ok(build_hash)) => {
                        let bh = &build_hash.clone();
                        self.post_download(bh);
                    }
                }
            }
            if self.download_finished_build_hash.is_some() {
                if let Ok(tried_prog) = self.download_prog.try_read() {
                    ui.add_space(10.);
                    ui.add(egui::widgets::ProgressBar::new(*tried_prog).show_percentage());
                }
            }

            ui.add_space(10.);
            if ui.button("Logout").clicked() {
                self.mutate_router("");
                self.account = None;
            }

            Ok(())
        })
        .inner
    }
    fn post_download(&mut self, build_hash: &str) {
        self.download_finished_build_hash = None;
        self.config.build_hash = build_hash.to_string();
    }
    fn download(&mut self) {
        let (sender, promise) = Promise::new();

        let prog_clone_1 = self.download_prog.clone();
        self.runtime.spawn(async move {
            println!("Download Started!");

            if let Some(user_dirs) = UserDirs::new() {
                if let Some(document_dir) = user_dirs.document_dir() {
                    let game_path = document_dir.join("RealmOfTheMadGod/Production/");
                    let platform = "rotmg-exalt-win-64";
                    let build_hash = exalta_core::misc::init(None, None).await?.build_hash;
                    let checksums =
                        exalta_core::download::request_checksums(&build_hash, platform).await?;
                    sender.send(
                        exalta_core::download::download_files_from_checksums(
                            &build_hash,
                            platform,
                            &game_path,
                            &checksums.files,
                            Some(prog_clone_1),
                        )
                        .await.map(|_| {
                            return build_hash
                        }),
                    );
                    println!("Download Ended!");
                }
            }
            Ok::<(), anyhow::Error>(())
        });

        self.download_finished_build_hash = Some(promise);
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
                            steam_id: Some(base64::encode(
                                &self.auth.guid.replace("steamworks:", ""),
                            )),
                            token: base64::encode(account.access_token.clone()),
                            token_timestamp: base64::encode(account.access_token_timestamp.clone()),
                            token_expiration: base64::encode(
                                account.access_token_expiration.clone(),
                            ),
                            env: 4,
                            server_name: String::new(),
                        })?
                    } else {
                        serde_json::to_string(&LaunchArgs {
                            platform: "Deca".to_string(),
                            guid: base64::encode(&self.auth.guid),
                            platform_token: None,
                            steam_id: None,
                            token: base64::encode(account.access_token.clone()),
                            token_timestamp: base64::encode(account.access_token_timestamp.clone()),
                            token_expiration: base64::encode(
                                account.access_token_expiration.clone(),
                            ),
                            env: 4,
                            server_name: String::new(),
                        })?
                    }
                    .replace("\"", "");
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
