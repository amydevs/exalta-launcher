use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]

#[derive(Default)]
pub struct SavedLauncherAuth {
    pub saved: Vec<LauncherAuth>,
    pub current: usize,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Default)]
pub struct LauncherAuth {
    pub guid: String,
    pub password: String,
}

pub struct ResultTimeWrapper {
    pub result: Result<(), Box<dyn std::error::Error>>,
    pub time: std::time::Instant,
}
impl Default for ResultTimeWrapper {
    fn default() -> Self {
        Self {
            result: Ok(()),
            time: std::time::Instant::now(),
        }
    }
}

pub fn get_device_token() -> String {
    use sha1::{Digest, Sha1};
    use smbioslib::*;
    let mut concat = String::new();

    if let Ok(data) = table_load_from_device() {
        if let Some(d) = data.first::<SMBiosBaseboardInformation>() {
            concat += &d.serial_number().to_string();
        }
        if let Some(d) = data.first::<SMBiosSystemInformation>() {
            concat += &d.serial_number().to_string();
        }
    }

    if concat.is_empty() {
        concat += "None0"
    }

    if let Ok(d) = get_product_id() {
        concat += &d;
    }

    println!("{}", concat);

    let mut hasher = Sha1::new();
    hasher.update(concat);
    format!("{:x}", &hasher.finalize())
}

#[cfg(windows)]
fn get_product_id() -> Result<String, Box<dyn std::error::Error>> {
    use wmi::*;

    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;
    let results: Vec<HashMap<String, Variant>> =
        wmi_con.raw_query("SELECT * FROM Win32_OperatingSystem")?;
    for os in results {
        if let Some(var) = os.get("SerialNumber") {
            if let Variant::String(s) = var {
                return Ok(s.clone());
            }
        }
    }

    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "SerialNumber not found",
    )))
}

#[cfg(target_os = "linux")]
fn get_product_id() -> Result<String, Box<dyn std::error::Error>> {
    use std::process::Command;

    use regex::Regex;

    let output = Command::new("wine")
        .args(&["wmic", "os", "get", "SerialNumber"])
        .output()?;
    let out = String::from_utf8_lossy(
        &output
            .stdout
            .clone()
            .iter()
            .filter(|e| **e != 0)
            .map(|e| *e)
            .collect::<Vec<u8>>(),
    )
    .to_string();

    if let Some(s) = Regex::new(r"SerialNumber\s*\r\n([^\s\\]*)")?.captures(&out) {
        if let Some(e) = s.get(1) {
            return Ok(e.as_str().to_string());
        }
    }

    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "SerialNumber not found",
    )))
}

pub fn with_index<T, F>(mut f: F) -> impl FnMut(&T) -> bool
where
    F: FnMut(usize, &T) -> bool,
{
    let mut i = 0;
    move |item| (f(i, item), i += 1).0
}
