use std::{path::Path, sync::Arc};

use eframe::egui::{self, Ui};
use poll_promise::Promise;
use std::process::Command;
use tokio::sync::RwLock;

use crate::{launchargs::LaunchArgs, main_ext::ResultTimeWrapper, ExaltaLauncher};

use super::Route;

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
            let play_button = egui::Button::new("Play");
            if ui
                .add_enabled(self.download_finished_build_hash.is_none(), play_button)
                .clicked()
            {
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
            let update_button = egui::Button::new("Update / Verify / Download Game");
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
                        self.run_res.result = Err(Box::new(
                            exalta_core::download::err::UpdateError("Download Failed!".to_string()),
                        ));
                        self.post_download("");
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
                self.router_path.set(Route::Login);
                self.account = None;
            }

            Ok(())
        })
        .inner
    }
    fn post_download(&mut self, build_hash: &str) {
        self.download_finished_build_hash = None;
        self.download_prog = Arc::new(RwLock::new(0.0));
        self.config.build_hash = build_hash.to_string();
        if self.config.save().is_ok() {
            println!("Saved build hash: {}", build_hash);
        }

        #[cfg(windows)]
        crate::registries::set_build_id(build_hash).ok();
    }
    fn download(&mut self) {
        let (sender, promise) = Promise::new();

        let prog_clone_1 = self.download_prog.clone();
        let game_folder_path = Path::new(&self.config.game_folder_path).to_path_buf();
        self.runtime.spawn(async move {
            println!("Download Started!");
            let game_path =
                game_folder_path.join(format!("{}/", exalta_core::BUILD_TYPE.read().await));
            let platform = "rotmg-exalt-win-64";
            let build_hash = exalta_core::misc::init(None, None).await?.build_hash;
            let checksums = exalta_core::download::request_checksums(&build_hash, platform).await?;
            sender.send(
                exalta_core::download::download_files_from_checksums(
                    &build_hash,
                    platform,
                    &game_path,
                    &checksums.files,
                    Some(prog_clone_1),
                )
                .await
                .map(|_| build_hash),
            );
            println!("Download Ended!");
            Ok::<(), anyhow::Error>(())
        });

        self.download_finished_build_hash = Some(promise);
    }
    fn load(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(account) = &self.account {
            let execpath = Path::new(&self.config.game_folder_path).join(format!(
                "{}/RotMG Exalt.exe",
                exalta_core::BUILD_TYPE.blocking_read()
            ));
            let args = if let Some(steam_creds) = &self.steam_credentials {
                serde_json::to_string(&LaunchArgs {
                    platform: "Steam".to_string(),
                    guid: base64::encode(&self.auth.guid),
                    platform_token: Some(base64::encode(&steam_creds.platform_token)),
                    steam_id: Some(base64::encode(self.auth.guid.replace("steamworks:", ""))),
                    token: base64::encode(account.access_token.clone()),
                    token_timestamp: base64::encode(account.access_token_timestamp.clone()),
                    token_expiration: base64::encode(account.access_token_expiration.clone()),
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
                    token_expiration: base64::encode(account.access_token_expiration.clone()),
                    env: 4,
                    server_name: String::new(),
                })?
            }
            .replace('"', "");
            println!("{}", args);

            #[cfg(target_os = "windows")]
            Command::new(execpath.to_str().unwrap())
                .args(&[format!("data:{}", args)])
                .spawn()?;

            #[cfg(target_os = "linux")]
            Command::new("sh")
                .args(&[
                    "-c",
                    &format!(
                        "wine \"{}\" \"{}\"",
                        execpath.to_str().unwrap(),
                        &format!("data:{}", args)
                    ),
                ])
                .spawn()
                .ok();
        }
        Ok(())
    }
}
