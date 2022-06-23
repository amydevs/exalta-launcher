use eframe::egui::{Ui, self};

use crate::ExaltaLauncher;

impl ExaltaLauncher {
    pub fn render_config(&mut self, ui: &mut Ui) -> Result<(), Box<dyn std::error::Error>> {
        let mut changed = false;
        ui.vertical_centered_justified(|ui| {
            let dark_clicked = ui.horizontal(|ui| {
                ui.label("Dark Mode:");
                ui.checkbox(&mut self.config.dark, "")
            }).inner.changed();
            if dark_clicked {
                changed = dark_clicked;
                if self.config.dark {
                    ui.ctx().set_visuals(egui::Visuals::dark());
                }
                else {
                    ui.ctx().set_visuals(egui::Visuals::light());
                }
            }

            if changed {
                self.config.save()?;
            };
            Ok(())
        }).inner        
    }
}