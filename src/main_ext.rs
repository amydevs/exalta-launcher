use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
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
