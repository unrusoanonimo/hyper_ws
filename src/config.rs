use std::fs::File;

use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    port: u16,
    log_level: Option<u8>,
    console: Option<bool>,
}
impl Config {
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn log_level(&self) -> u8 {
        self.log_level.unwrap_or_default()
    }
    pub fn console(&self) -> bool {
        self.console.unwrap_or(false)
    }
}
const PATH: &str = "config.json";
pub static CONFIG: Lazy<Config> =
    Lazy::new(|| serde_json::from_reader(File::open(PATH).unwrap()).unwrap());
