use std::sync::mpsc::{Sender, Receiver, channel};

use directories::UserDirs;
use exalta_core::{ExaltaClient, auth::{AuthController, account::Account}};
use launchargs::LaunchArgs;
use serde::{Serialize, Deserialize};
use tokio::{process::Command, runtime::Runtime};

mod login;
mod play;

mod args;
mod launchargs;

use eframe::egui::{self, Response};

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
    username: String,
    password: String
}
struct ResultTimeWrapper {
    result: Result<(), Box<dyn std::error::Error>>,
    time: std::time::Instant
}
struct ExaltaLauncher {
    auth: LauncherAuth,
    auth_con: Option<AuthController>,

    entry: keyring::Entry,
    runtime: Runtime,

    run_res: ResultTimeWrapper,
}

impl Default for ExaltaLauncher {
    fn default() -> Self {
        let entry = keyring::Entry::new(&"exalt", &"jsondata");
        let mut auth = LauncherAuth {
            username: String::new(),
            password: String::new()
        };
        if let Some(val) = entry.get_password().ok() {
            if let Some(foundauth) = serde_json::from_str::<LauncherAuth>(&val).ok() {
                auth = foundauth
            };
        };

        Self {
            auth,
            auth_con: None,
            entry,
            runtime: Runtime::new().unwrap(),
            run_res: ResultTimeWrapper {
                result: Ok(()),
                time: std::time::Instant::now()
            }
        }
    }
}

impl eframe::App for ExaltaLauncher {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Err(err) = egui::CentralPanel::default().show(ctx, |ui| -> Result<(), Box<dyn std::error::Error>> {
            ui.heading("Exalta Launcher");

            // play
            if self.auth_con.is_some() {
                self.render_play(ui)
            }
            // login
            else {
                self.render_login(ui)
            }
        }).inner {
            self.run_res = ResultTimeWrapper {
                result: Err(err),
                time: std::time::Instant::now()
            };
        };

        if let Err(e) = &self.run_res.result {
            if &self.run_res.time.elapsed().as_secs() < &5 {
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
        let auth_con = self.runtime.block_on(
            ExaltaClient::new().unwrap().login(
                &self.auth.username.as_str(), &self.auth.password.as_str()
            )
        )?;

        self.run_res.result = Ok(());
        self.auth_con = Some(auth_con);
        
        if let Some(json) = serde_json::to_string(&self.auth).ok() {
            self.entry.set_password(json.as_str())?;
        }
        Ok(())
    }
}