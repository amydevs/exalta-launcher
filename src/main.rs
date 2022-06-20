#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use exalta_core::auth::{account::Account, *};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

mod login;
mod play;

mod args;
mod launchargs;

#[cfg(windows)]
mod registries;

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Exalta Launcher",
        options,
        Box::new(|_cc| Box::new(ExaltaLauncher::default())),
    );
}

#[derive(Serialize, Deserialize, Clone)]
struct LauncherAuth {
    guid: String,
    password: String,
}
struct ResultTimeWrapper {
    result: Result<(), Box<dyn std::error::Error>>,
    time: std::time::Instant,
}
struct ExaltaLauncher {
    auth: LauncherAuth,
    auth_save: bool,
    account: Option<Account>,

    steam_flag: bool,

    entry: keyring::Entry,
    runtime: Runtime,

    run_res: ResultTimeWrapper,
}

impl Default for ExaltaLauncher {
    fn default() -> Self {
        let entry = keyring::Entry::new(&"exalt", &"jsondata");

        let mut run_res = ResultTimeWrapper {
            result: Ok(()),
            time: std::time::Instant::now(),
        };

        let runtime = Runtime::new().unwrap();

        #[cfg(windows)]
        {
            use registries::UpdateError;

            let regirunner = || -> Result<(), Box<dyn std::error::Error>> {
                let buildid = crate::registries::get_build_id()?;
                let buildhash = runtime
                    .block_on(exalta_core::misc::init(None, None))?
                    .build_hash;
                if buildid != buildhash {
                    return Err(Box::new(UpdateError(String::from(
                        "An update for the game is available, please run the official launcher to update the game first."
                    ))));
                }
                Ok(())
            };

            run_res = ResultTimeWrapper {
                result: regirunner().map_err(|x| {
                    if x.is::<UpdateError>() {
                        x
                    } else {
                        Box::new(UpdateError(String::from("Failed to check for updates.")))
                    }
                }),
                time: std::time::Instant::now(),
            };
        }

        let mut self_inst = Self {
            auth: LauncherAuth {
                guid: String::new(),
                password: String::new(),
            },
            auth_save: true,
            account: None,
            steam_flag: std::env::args().collect::<Vec<String>>().into_iter().find(|x| x.to_lowercase() == "--steam" || x.to_lowercase() == "-s" ).is_some(),
            entry,
            runtime,
            run_res,
        };

        if self_inst.steam_flag {
            self_inst.login().ok();
        }
        if let Some(val) = self_inst.entry.get_password().ok() {
            if let Some(foundauth) = serde_json::from_str::<LauncherAuth>(&val).ok() {
                self_inst.auth = foundauth;
                self_inst.login().ok();
            };
        };

        self_inst
    }
}

impl eframe::App for ExaltaLauncher {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(2.0);
        if let Err(err) = egui::CentralPanel::default()
            .show(ctx, |ui| -> Result<(), Box<dyn std::error::Error>> {
                ui.heading("Exalta Launcher");

                // play
                if self.account.is_some() {
                    self.render_play(ui)
                }
                // login
                else {
                    self.render_login(ui)
                }
            })
            .inner
        {
            self.run_res = ResultTimeWrapper {
                result: Err(err),
                time: std::time::Instant::now(),
            };
        };

        if let Err(e) = &self.run_res.result {
            if &self.run_res.time.elapsed().as_secs() < &8 {
                egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.label(e.to_string());
                    });
                });
            }
        }
    }
}
impl ExaltaLauncher {
    fn login(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.steam_flag {
            use ::steamworks::Client;
            let (client, _) = Client::init_app(200210)?;
            self.auth.guid = format!("steamworks:{}", client.user().steam_id().raw().to_string());

            let session_ticket = String::from_utf8_lossy(&client.user().authentication_session_ticket().1).to_string();
            self.account = Some(self.runtime.block_on(
                request_account(&AuthInfo::default().session_token(&session_ticket))
            )?);
        }
        else {
            if !self.auth_save {
                self.entry.delete_password().ok();
            }
            let acc = self.runtime.block_on(request_account(
                &AuthInfo::default()
                    .username_password(&self.auth.guid.as_str(), &self.auth.password.as_str()),
            ))?;
    
            self.account = Some(acc);
    
            if self.auth_save {
                if let Ok(json) = serde_json::to_string(&self.auth) {
                    self.entry.set_password(json.as_str()).ok();
                }
            }
    
        }
        
        Ok(())
    }
}
