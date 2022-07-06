use eframe::egui::{self, Ui};

use crate::ExaltaLauncher;

impl ExaltaLauncher {
    pub fn render_config(&mut self, ui: &mut Ui) -> Result<(), Box<dyn std::error::Error>> {
        let mut changed = false;
        egui::Grid::new("options_grid")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {

            ui.label("Dark Mode:");
            let dark_clicked = ui.checkbox(&mut self.config.dark, "").changed();
            if dark_clicked {
                changed = dark_clicked;
                if self.config.dark {
                    ui.ctx().set_visuals(egui::Visuals::dark());
                } else {
                    ui.ctx().set_visuals(egui::Visuals::light());
                }
            }
            ui.end_row();

            ui.label("Game Install Path:");
            ui.horizontal(|ui| {
                if ui.button("🗀").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.config.game_folder_path = path.display().to_string();
                        changed = true;
                    }
                }
                if ui.button("⟳").clicked() {
                    if let Some(detected_loc) = super::AppConfig::get_default_game_location() {
                        self.config.game_folder_path = detected_loc.display().to_string();
                    }
                    else {
                        self.config.game_folder_path = String::new();
                    }
                    changed = true;
                }
                if ui.add(egui::TextEdit::singleline(&mut self.config.game_folder_path).hint_text("Write something here")).changed() {
                    changed = true;
                };
            });
            ui.end_row();
        });

        if changed {
            self.config.save()?;
        };
        Ok(())
    }
}
