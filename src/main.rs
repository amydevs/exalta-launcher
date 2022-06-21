#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use exalta_core::auth::{account::Account, *};
use serde::{Deserialize, Serialize};
use ::steamworks::{AuthTicket, AuthSessionTicketResponse, Callback, ValidateAuthTicketResponse};
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

    steam_client: Option<(::steamworks::Client, ::steamworks::SingleClient)>,

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
            steam_client:
                if std::env::args().collect::<Vec<String>>().into_iter().find(|x| x.to_lowercase() == "--steam" || x.to_lowercase() == "-s" ).is_some() {
                    ::steamworks::Client::init_app(200210).ok()
                }
                else {
                  None  
                },
            entry,
            runtime,
            run_res,
        };

        if let Some(client) = &self_inst.steam_client {
            exalta_core::set_steamid_game_net_play_platform(&client.0.user().steam_id().raw().to_string());
            self_inst.login().unwrap();
        }
        else {
            if let Some(val) = self_inst.entry.get_password().ok() {
                if let Some(foundauth) = serde_json::from_str::<LauncherAuth>(&val).ok() {
                    self_inst.auth = foundauth;
                    self_inst.login().ok();
                };
            };
        }
        

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
        if let Some((client, single)) = &self.steam_client {
            self.auth.guid = format!("steamworks:{}", client.user().steam_id().raw().to_string());
            let user = client.user();

            let _cb = client.register_callback(|v: AuthSessionTicketResponse| { 
                println!("Got Response from Steam: {:?}", v.result)
            });

            let (auth, ticket) = user.authentication_session_ticket();

            for _ in 0..20 {
                single.run_callbacks();
                ::std::thread::sleep(::std::time::Duration::from_millis(50));
            }

            println!("END");
            self.account = Some(self.runtime.block_on(
                request_account(&AuthInfo::default().session_token(&encode_hex(&ticket)))
            )?);

            user.cancel_authentication_ticket(auth);
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

fn encode_hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}