use eframe::egui::Ui;

use crate::ExaltaLauncher;

impl ExaltaLauncher {
    pub fn render_config(&mut self, ui: &mut Ui) -> Result<(), Box<dyn std::error::Error>> {
        ui.vertical_centered_justified(|ui| {
            ui.horizontal(|ui| {
                ui.label("Dark Mode:");
                ui.checkbox(&mut self.config.dark, "")
            });
            Ok(())
        }).inner
    }
}