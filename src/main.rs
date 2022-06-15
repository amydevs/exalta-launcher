use std::sync::mpsc::{Sender, Receiver};

use directories::UserDirs;
use exalta_core::{ExaltaClient, auth::{AuthController, account::Account}};
use launchargs::LaunchArgs;
use tokio::{process::Command, runtime::Runtime};

mod args;
mod launchargs;


use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Exalta Launcher",
        options,
        Box::new(|_cc| Box::new(ExaltaLauncher::default())),
    );
}

struct ExaltaLauncher {
    username: String,
    password: String,
    auth_con: Option<AuthController>
}

impl Default for ExaltaLauncher {
    fn default() -> Self {
        Self {
            username: "".to_owned(),
            password: "".to_owned(),
            auth_con: None
        }
    }
}

impl eframe::App for ExaltaLauncher {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Exalta Launcher");

            let rt  = Runtime::new().unwrap();

            // play
            if self.auth_con.is_some() {
                ui.horizontal_centered(|ui| {
                    if ui.button("Logout").clicked() {
                        self.auth_con = None;
                    }
                    if ui.button("Play").clicked() {
                        if rt.block_on(self.auth_con.as_ref().unwrap().verify()).is_ok() {
                            load(&self.auth_con.as_ref().unwrap().account, self.username.as_str()).ok();
                        }
                        else if let Some(resauth) = rt.block_on(
                            ExaltaClient::new().unwrap().login(
                                self.username.as_str(), self.password.as_str()
                        )).ok() {
                            self.auth_con = Some(resauth);
                        }
                        else {
                            self.auth_con = None;
                        }
                    }
                });
                // ui.horizontal(|ui| {
                //     if ui.button("Logout").clicked() {
                //         self.auth_con = None;
                //     }
                //     if ui.button("Play").clicked() {
                //         self.auth_con.
                //     }
                // });
            }
            // login
            else {
                ui.vertical_centered_justified(|ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.label("Username: ");
                        ui.text_edit_singleline(&mut self.username);
                    });
                    ui.add_space(10.);

                    ui.vertical_centered_justified(|ui| {
                        ui.label("Password: ");
                        ui.add(egui::TextEdit::singleline(&mut self.password).password(true));
                    });
                    ui.add_space(10.);

                    if ui.button("Login").clicked() {
                        if let Some(resauth) = rt.block_on(
                            ExaltaClient::new().unwrap().login(
                                self.username.as_str(), self.password.as_str()
                            )
                        ).ok() {
                            self.auth_con = Some(resauth);
                        }
                        else {
                            println!("Login failed");
                        }
                    }
                });
            }
        });
    }
}

// #[tokio::main]
fn load(account: &Account, username: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(user_dirs) = UserDirs::new() {
        if let Some(document_dir) = user_dirs.document_dir() {
            let execpath = document_dir.join("RealmOfTheMadGod/Production/RotMG Exalt.exe");
            let args = serde_json::to_string(&LaunchArgs {
                platform: "Deca".to_string(),
                guid: base64::encode(username),
                token: base64::encode(account.access_token.clone()),
                token_timestamp: base64::encode(account.access_token_timestamp.clone()),
                token_expiration: base64::encode(account.access_token_expiration.clone()),
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