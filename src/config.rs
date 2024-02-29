use std::fs::File;

use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub port: u16,
    pub log_level: u8,
    pub console: bool,
    pub origin: String,
}

const PATH: &str = "config.json";
pub static CONFIG: Lazy<Config> =
    Lazy::new(|| serde_json::from_reader(File::open(PATH).unwrap()).unwrap());
