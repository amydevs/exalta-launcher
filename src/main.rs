use std::sync::Arc;

use exalta_core::auth::{account::Account, *};
use main_ext::{LauncherAuth, ResultTimeWrapper};
use poll_promise::Promise;
use tokio::{runtime::Runtime, sync::RwLock};

mod main_ext;

mod login;
mod play;

mod args;
mod config;
mod launchargs;

#[cfg(windows)]
mod registries;
mod update;

use eframe::egui;
use update::UpdateError;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Exalta Launcher",
        options,
        Box::new(|_cc| {
            let config = config::AppConfig::load().unwrap_or_default();
            if config.dark {
                _cc.egui_ctx.set_visuals(egui::Visuals::dark());
            } else {
                _cc.egui_ctx.set_visuals(egui::Visuals::light());
            }
            Box::new(ExaltaLauncher::default())
        }),
    );
}

struct ExaltaLauncher {
    auth: LauncherAuth,
    account: Option<Account>,

    #[cfg(feature = "steam")]
    steam_client: Option<(::steamworks::Client, ::steamworks::SingleClient)>,
    steam_credentials: Option<steamworks::Credentials>,

    entry: keyring::Entry,
    runtime: Runtime,

    run_res: ResultTimeWrapper,

    router_path: [&'static str; 2],
    config: config::AppConfig,

    download_finished_build_hash: Option<Promise<anyhow::Result<String>>>,
    download_prog: Arc<RwLock<f32>>,
}

impl Default for ExaltaLauncher {
    fn default() -> Self {
        let entry = keyring::Entry::new(&"exalt", &"jsondata");

        let mut run_res = ResultTimeWrapper::default();

        let runtime = Runtime::new().unwrap();

        let mut config = config::AppConfig::load().unwrap_or_default();

        {
            use update::UpdateError;

            let update_error = Box::new(UpdateError(String::from(
                "An update for the game is available, please run the official launcher to update the game first."
            )));

            let update_runner = || -> Result<(), Box<dyn std::error::Error>> {
                #[cfg(windows)]
                let registry_build_hash = crate::registries::get_build_id()?;
                #[cfg(not(windows))]
                let registry_build_hash = String::new();

                let buildhash = runtime
                    .block_on(exalta_core::misc::init(None, None))?
                    .build_hash;

                println!(
                    "Old: {} == New: {}",
                    if registry_build_hash.is_empty() {
                        &config.build_hash
                    } else {
                        &registry_build_hash
                    },
                    buildhash
                );

                #[cfg(windows)]
                if config.build_hash.is_empty() {
                    if &registry_build_hash != &buildhash {
                        return Err(update_error);
                    } else {
                        config.build_hash = buildhash;
                        config.save()?;
                    }
                } else {
                    if &registry_build_hash == &buildhash {
                        config.build_hash = buildhash;
                        config.save()?;
                    } else if &config.build_hash != &buildhash {
                        return Err(update_error);
                    }
                }

                #[cfg(not(windows))]
                if config.build_hash != buildhash {
                    return Err(update_error);
                }

                Ok(())
            };

            run_res = ResultTimeWrapper::default();
            run_res.result = update_runner().map_err(|x| {
                if x.is::<UpdateError>() {
                    x
                } else {
                    Box::new(UpdateError(String::from("Failed to check for updates.")))
                }
            });
        }

        let mut self_inst = Self {
            auth: LauncherAuth {
                guid: String::new(),
                password: String::new(),
            },
            account: None,

            #[cfg(feature = "steam")]
            steam_client: ::steamworks::Client::init_app(200210).ok(),
            steam_credentials: None,

            entry,
            runtime,
            run_res,

            router_path: [""; 2],
            config,

            download_finished_build_hash: None,
            download_prog: Arc::new(RwLock::new(0.0)),
        };

        #[cfg(feature = "steam")]
        if let Some(client) = &self_inst.steam_client {
            exalta_core::set_steamid_game_net_play_platform(
                &client.0.user().steam_id().raw().to_string(),
            );
            let res = self_inst.login();
            if self_inst.run_res.result.is_ok() {
                self_inst.run_res = ResultTimeWrapper::default();
                self_inst.run_res.result = res;
            }
        }

        #[cfg(not(feature = "steam"))]
        if let Ok(val) = self_inst.entry.get_password() {
            if let Ok(foundauth) = serde_json::from_str::<LauncherAuth>(&val) {
                self_inst.auth = foundauth;

                let res = self_inst.login();
                if self_inst.run_res.result.is_ok() {
                    self_inst.run_res = ResultTimeWrapper::default();
                    self_inst.run_res.result = res;
                }
            };
        };

        self_inst
    }
}

impl eframe::App for ExaltaLauncher {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(2.0);
        egui::TopBottomPanel::top("top panel").show(ctx, |ui| {
            use egui::{Button, Rect, RichText, Vec2};
            ui.heading("Exalta Launcher");
            let rect = ui.max_rect();

            let right_top = rect.right_top() + Vec2 { x: -4.5, y: 1. };
            let settings_resp = ui.put(
                Rect::from_points(&[right_top]),
                Button::new(RichText::new("\u{2699}")).frame(false),
            );
            if settings_resp.clicked() {
                if *self.router_path.last().unwrap() == "config" {
                    self.mutate_router_back();
                } else {
                    self.mutate_router("config");
                }
            }
        });
        if let Err(err) = egui::CentralPanel::default()
            .show(ctx, |ui| -> Result<(), Box<dyn std::error::Error>> {
                match *self.router_path.last().unwrap() {
                    "play" => {
                        if self.account.is_some() {
                            self.render_play(ui)
                        } else {
                            todo!()
                        }
                    }
                    "config" => self.render_config(ui),
                    _ => self.render_login(ui),
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
    #[cfg(feature = "steam")]
    fn login(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some((client, single)) = &self.steam_client {
            self.auth.guid = format!("steamworks:{}", client.user().steam_id().raw().to_string());
            let user = client.user();

            let _cb = client.register_callback(|v: ::steamworks::AuthSessionTicketResponse| {
                println!("Got Response from Steam: {:?}", v.result)
            });

            let (auth, ticket) = user.authentication_session_ticket();

            for _ in 0..20 {
                single.run_callbacks();
                ::std::thread::sleep(::std::time::Duration::from_millis(50));
            }

            println!("END");
            let credentials =
                self.runtime
                    .block_on(exalta_core::auth::steamworks::request_credentials(
                        &exalta_core::auth::steamworks::encode_hex(&ticket),
                    ))?;
            self.steam_credentials = Some(credentials.clone());
            self.account = Some(self.runtime.block_on(request_account(
                &AuthInfo::default().steamworks_credentials(credentials),
            ))?);

            user.cancel_authentication_ticket(auth);
        }
        self.run_inits();
        Ok(())
    }
    #[cfg(not(feature = "steam"))]
    fn login(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.config.save_login {
            self.entry.delete_password().ok();
        }
        let acc = self.runtime.block_on(request_account(
            &AuthInfo::default()
                .username_password(&self.auth.guid.as_str(), &self.auth.password.as_str()),
        ))?;

        self.account = Some(acc);
        self.mutate_router("play");

        if self.config.save_login {
            if let Ok(json) = serde_json::to_string(&self.auth) {
                self.entry.set_password(json.as_str()).ok();
            }
        }
        self.run_inits();
        Ok(())
    }
    fn run_inits(&mut self) {
        if let Some(account) = &self.account {
            let access_token = account.access_token.clone();
            self.runtime.spawn(async move {
                exalta_core::misc::init(Some("rotmg"), Some(&access_token))
                    .await
                    .ok();
                exalta_core::misc::init(None, Some(&access_token))
                    .await
                    .ok();
            });
        }
    }

    pub fn mutate_router(&mut self, route: &'static str) {
        self.router_path.rotate_left(1);
        *self.router_path.last_mut().unwrap() = route;
    }
    pub fn mutate_router_back(&mut self) {
        self.router_path.rotate_right(1);
    }
}
