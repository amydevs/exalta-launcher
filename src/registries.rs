use std::collections::HashMap;

use winreg::{enums::*, RegKey};

pub fn get_product_id() -> Result<String, Box<dyn std::error::Error>> {
    use wmi::*;

    
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;
    let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query("SELECT * FROM Win32_OperatingSystem")?;
    for os in results {
        if let Some(var) = os.get("SerialNumber") {
            if let Variant::String(s) = var {
                return Ok(s.clone())
            }
        }
    }

    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "SerialNumber not found",
    )))
}

pub fn get_build_id() -> Result<String, Box<dyn std::error::Error>> {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let launcherloc =
        hklm.open_subkey("SOFTWARE\\DECA Live Operations GmbH\\RotMG Exalt Launcher")?;
    let mut found_string = None;
    for row in launcherloc.enum_values() {
        if let Some((key, val)) = row.ok() {
            if key.contains("buildId_v2_Production") {
                found_string = Some(String::from_utf8_lossy(&val.bytes).to_string());
            }
        }
    }
    if let Some(val) = found_string {
        Ok(val.replace("\u{0}", ""))
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "build id not found",
        )))
    }
}

pub fn set_build_id(build_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let launcherloc =
        hklm.open_subkey("SOFTWARE\\DECA Live Operations GmbH\\RotMG Exalt Launcher")?;
    let mut found_key = None;
    for row in launcherloc.enum_keys() {
        if let Some(key) = row.ok() {
            if key.contains("buildId_v2_Production") {
                found_key = Some(key);
            }
        }
    }
    if let Some(found_key) = found_key {
        launcherloc.set_value(found_key, &format!("{}\u{0}", build_id))?;
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "build id not found",
        )))
    }
}
