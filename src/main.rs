use std::sync::Arc;

use exalta_core::{
    auth::{account::Account, *},
    download::err::UpdateError,
};
use main_ext::{LauncherAuth, ResultTimeWrapper, get_device_token, SavedLauncherAuth};
use pages::{config, HistoryVec, Route};
use poll_promise::Promise;
use tokio::{runtime::Runtime, sync::RwLock};

mod main_ext;

mod args;
mod launchargs;
mod pages;

#[cfg(windows)]
mod registries;

use eframe::egui::{self};

#[cfg(not(feature = "steam"))]
static APP_NAME: &str = "Exalta Launcher";
#[cfg(feature = "steam")]
static APP_NAME: &str = "Exalta Launcher - Steam Edition";

fn main() {
    let options = eframe::NativeOptions::default();

    exalta_core::set_client_token(&get_device_token());

    eframe::run_native(
        APP_NAME,
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
    saved_auth: SavedLauncherAuth,
    account: Option<Account>,

    #[cfg(feature = "steam")]
    steam_client: Option<(::steamworks::Client, ::steamworks::SingleClient)>,
    steam_credentials: Option<steamworks::Credentials>,

    entry: keyring::Entry,
    runtime: Runtime,

    run_res: ResultTimeWrapper,

    router_path: HistoryVec<Route>,
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
            let update_error = Box::new(UpdateError(String::from(
                "An update for the game seems to be available, please click on the \"Update / Verify / Download Game\" button once you've logged in."
            )));

            let update_runner = || -> Result<(), Box<dyn std::error::Error>> {
                #[cfg(windows)]
                let registry_build_hash = crate::registries::get_build_id().unwrap_or_default();
                #[cfg(not(windows))]
                let registry_build_hash = String::new();

                let buildhash = runtime
                    .block_on(exalta_core::misc::init(None, None))?
                    .build_hash;

                println!(
                    "Old: {:X?} == New: {:X?}",
                    if registry_build_hash.is_empty() {
                        &config.build_hash
                    } else {
                        &registry_build_hash
                    },
                    &buildhash
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
            saved_auth: SavedLauncherAuth::default(),
            account: None,

            #[cfg(feature = "steam")]
            steam_client: ::steamworks::Client::init_app(200210).ok(),
            steam_credentials: None,

            entry,
            runtime,
            run_res,

            router_path: HistoryVec::new(Route::Login),
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
            if let Ok(foundauthvec) = serde_json::from_str::<SavedLauncherAuth>(&val) {
                self_inst.saved_auth = foundauthvec;
                if self_inst.config.save_login {
                    if let Some(foundauth) = self_inst.saved_auth.saved.get(self_inst.saved_auth.current) {
                        self_inst.auth = foundauth.clone();
                    }
                    
                    let res = self_inst.login();
                    if self_inst.run_res.result.is_ok() {
                        self_inst.run_res = ResultTimeWrapper::default();
                        self_inst.run_res.result = res;
                    }
                }
            };
        };

        self_inst
    }
}

impl eframe::App for ExaltaLauncher {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.8);
        egui::TopBottomPanel::top("top panel").show(ctx, |ui| {
            use egui::{Button, Rect, RichText, Vec2};
            ui.heading(APP_NAME);
            let rect = ui.max_rect();

            let right_top = rect.right_top() + Vec2 { x: -4.5, y: 1. };
            let settings_resp = ui.put(
                Rect::from_points(&[right_top]),
                Button::new(RichText::new("\u{2699}")).frame(false),
            );
            if settings_resp.clicked() {
                if matches!(self.router_path.get(), Route::Config) {
                    self.router_path.revert();
                } else {
                    self.router_path.set(Route::Config);
                }
            }
        });
        if let Err(err) = egui::CentralPanel::default()
            .show(ctx, |ui| -> Result<(), Box<dyn std::error::Error>> {
                match self.router_path.get() {
                    Route::Play => {
                        if self.account.is_some() {
                            self.render_play(ui)
                        } else {
                            self.router_path.revert();
                            Ok(())
                        }
                    }
                    Route::Config => self.render_config(ui),
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
