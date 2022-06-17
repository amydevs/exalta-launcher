use std::{fmt, error::Error, string};

use winreg::{enums::*, RegKey};

pub fn get_build_id() -> Result<String, Box<dyn std::error::Error>> {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let launcherloc = hklm.open_subkey("SOFTWARE\\DECA Live Operations GmbH\\RotMG Exalt Launcher")?;
    let mut found_string = None;
    for row in launcherloc.enum_values() {
        if let Some((key, val)) = row.ok() {
            if key.contains("buildId") {
                found_string = Some(String::from_utf8_lossy(&val.bytes).to_string());
            }
        }
    };
    if let Some(val) = found_string {
        Ok(val)
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "build id not found")))
    }
}

#[derive(Debug)]
pub struct RegistryError(pub String);

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RegistryError: {}", self.0)
    }
}

impl Error for RegistryError {}

#[derive(Debug)]
pub struct UpdateError(pub String);

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for UpdateError {}