use super::Route;
use eframe::egui::{self, Ui};
use exalta_core::auth::{err::AuthError, request_account, request_forgot_password, AuthInfo};
use regex::Regex;

use crate::{ExaltaLauncher, main_ext::{LauncherAuth, with_index}};

impl ExaltaLauncher {
    pub fn render_login(&mut self, ui: &mut Ui) -> Result<(), Box<dyn std::error::Error>> {
        ui.vertical_centered_justified(|ui| {
            #[cfg(not(feature = "steam"))]
            {
                ui.vertical_centered_justified(|ui| -> Result<(), Box<dyn std::error::Error>> {
                    ui.label("Username: ");
                    let re = ui.text_edit_singleline(&mut self.auth.guid);
                    if re.lost_focus() && re.ctx.input().key_pressed(egui::Key::Enter) {
                        self.login()?;
                    }
                    Ok(())
                })
                .inner?;
                ui.add_space(10.);
                ui.vertical_centered_justified(|ui| -> Result<(), Box<dyn std::error::Error>> {
                    ui.label("Password: ");
                    let re = ui.add(egui::TextEdit::singleline(&mut self.auth.password).password(true));
                    if re.lost_focus() && re.ctx.input().key_pressed(egui::Key::Enter) {
                        self.login()?;
                    }
                    Ok(())
                })
                .inner?;
                ui.add_space(10.);

                ui.horizontal_wrapped(|ui| -> Result<(), Box<dyn std::error::Error>> {
                    if ui
                        .checkbox(&mut self.config.save_login, "Save Login")
                        .changed()
                    {
                        self.config.save()?;
                    }

                    let mut saved_auth_changed = false;
                    egui::ComboBox::from_id_source("saved_combo")
                    .selected_text(self.saved_auth.saved.iter().map(|e| e.guid.as_str()).nth(self.saved_auth.current).unwrap_or("Saved Logins"))
                    .show_ui(ui, |ui| {
                        egui::Grid::new("saved_grid").num_columns(2).show(ui, |ui| {
                            self.saved_auth.saved.retain(with_index(|i, auth: &LauncherAuth| {
                                let mut retained = true;
                                if ui.selectable_value(&mut self.saved_auth.current, i, &auth.guid).clicked() {
                                    self.auth = auth.clone();
                                    saved_auth_changed = true;
                                };
                                if ui.button("❌").clicked() {
                                    retained = false;
                                    if i == self.saved_auth.current {
                                        self.saved_auth.current -= 1;
                                        saved_auth_changed = true;
                                    }
                                }
                                ui.end_row();
                                retained
                            }));
                        });
                    });
                    if saved_auth_changed {
                        if let Ok(json) = serde_json::to_string(&self.saved_auth) {
                            self.entry.set_password(json.as_str())?;
                        }
                    }
                    Ok(())
                })
                .inner?;

                ui.add_space(10.);
                if ui.button("Reset Password").clicked() {
                    self.reset_password()?;
                }
            }

            ui.add_space(10.);
            if ui.button("Login").clicked() {
                self.login()?;
            }
            Ok(())
        })
        .inner
    }

    #[cfg(feature = "steam")]
    pub fn login(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        use exalta_core::auth::steamworks;
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
                    .block_on(steamworks::request_credentials(&steamworks::encode_hex(
                        &ticket,
                    )))?;
            self.steam_credentials = Some(credentials.clone());
            self.account = Some(self.runtime.block_on(request_account(
                &AuthInfo::default().steamworks_credentials(credentials),
            ))?);
            self.router_path.set(Route::Play);

            user.cancel_authentication_ticket(auth);
        }
        self.run_inits();
        Ok(())
    }
    #[cfg(not(feature = "steam"))]
    pub fn login(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let acc = self.runtime.block_on(request_account(
            &AuthInfo::default()
                .username_password(&self.auth.guid.as_str(), &self.auth.password.as_str()),
        ))?;

        self.account = Some(acc);
        self.router_path.set(Route::Play);

        if self.config.save_login {
            if let Some(existing_auth) = self.saved_auth.saved.iter().position(|e| e.guid == self.auth.guid) {
                self.saved_auth.saved.remove(existing_auth);
                
            }
            self.saved_auth.saved.push(self.auth.clone());
            self.saved_auth.current = self.saved_auth.saved.len() - 1;
            if let Ok(json) = serde_json::to_string(&self.saved_auth) {
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

    fn reset_password(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let email_regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )?;
        if email_regex.is_match(&self.auth.guid) {
            self.runtime
                .block_on(request_forgot_password(&self.auth.guid))?;
        } else {
            return Err(Box::new(AuthError(format!(
                "{} is not a valid email!",
                self.auth.guid
            ))));
        }

        Ok(())
    }
}
