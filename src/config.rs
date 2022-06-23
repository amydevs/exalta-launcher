use std::{fs::File, path::{Path, PathBuf}, io::{Read, Write}};

use directories::{BaseDirs, UserDirs, ProjectDirs};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub dark: bool
}
impl Default for AppConfig {
    fn default() -> Self {
        Self { 
            dark: false
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
        let config= Self::load_config(&Self::get_location()?)?;
        Ok(config)
    }
}