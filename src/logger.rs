use log::LevelFilter;
use std::{fs::File, io::Write, path::PathBuf};

use crate::config::CONFIG;

const LOG_DIR: &str = "logs";
pub fn setup() {
    let date = chrono::offset::Utc::now();
    let mut path = PathBuf::from(LOG_DIR);

    assert!(path.exists(), "There must be a logs directory");
    path.push(date.format("%Y-%m-%d %H.%M.%S UTC.log").to_string());
    assert!(!path.exists());

    let mut builder = env_logger::builder();
    if !CONFIG.console() {
        let log_file: Box<dyn Write + Send + 'static> = Box::new(File::create(path).unwrap());
        builder.target(env_logger::Target::Pipe(log_file));
    }
    builder.filter_level(filter_from_usize(CONFIG.log_level()).unwrap());
    builder.init();
}
fn filter_from_usize(u: u8) -> Option<LevelFilter> {
    match u {
        0 => Some(LevelFilter::Off),
        1 => Some(LevelFilter::Error),
        2 => Some(LevelFilter::Warn),
        3 => Some(LevelFilter::Info),
        4 => Some(LevelFilter::Debug),
        5 => Some(LevelFilter::Trace),
        _ => None,
    }
}
