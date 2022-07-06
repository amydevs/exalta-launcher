use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use directories::{ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};

mod gui;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub dark: bool,
    pub save_login: bool,
    pub build_hash: String,
    pub game_folder_path: String
}
impl Default for AppConfig {
    fn default() -> Self {
        let mut game_folder_path = String::new();
        if let Some(game_loc_detected) = Self::get_default_game_location() {
            game_folder_path = game_loc_detected.display().to_string();
        }
        Self {
            dark: false,
            save_login: true,
            build_hash: String::new(),
            game_folder_path: game_folder_path
        }
    }
}

impl AppConfig {
    fn get_location() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let location = match ProjectDirs::from("com", "AyanAmy", "Exalta") {
            Some(v) => {
                std::fs::create_dir_all(&v.config_dir())?;
                v.config_dir().to_path_buf()
            }
            None => std::env::current_dir()?,
        };

        Ok(location.join("config.json"))
    }

    fn load_config(location: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = String::new();
        let mut file = File::open(location)?;
        file.read_to_string(&mut config)?;

        let cfg = serde_json::from_str(&config)?;
        Ok(cfg)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = serde_json::to_string(&self)?;
        let location = Self::get_location()?;

        let mut file = File::create(location)?;
        file.write_all(config.as_bytes())?;

        Ok(())
    }
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Self::load_config(&Self::get_location()?)?;
        Ok(config)
    }

    pub fn get_default_game_location() -> Option<PathBuf> {
        Some(
            UserDirs::new()?
            .document_dir()?
            .join("RealmOfTheMadGod/")
        )
    }
}
