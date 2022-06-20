use eframe::egui::{self, Ui};

use crate::ExaltaLauncher;

impl ExaltaLauncher {
    pub fn render_login(&mut self, ui: &mut Ui) -> Result<(), Box<dyn std::error::Error>> {
        ui.vertical_centered_justified(|ui| {
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
            ui.vertical_centered_justified(|ui| {
                ui.checkbox(&mut self.auth_save, "Save Login");
            });
            ui.add_space(10.);
            if ui.button("Login").clicked() {
                self.login()?;
            }
            Ok(())
        })
        .inner
    }
}
