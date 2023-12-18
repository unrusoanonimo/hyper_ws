use std::fmt::Display;

use tokio::sync::Mutex;

pub mod ip_info;
pub mod user;

pub use ip_info::IpInfoModule;

pub const ATENDA_SQLITE_PATH: &str = "./data/atenda.sqlite";

pub struct AppModules {
    pub ip_info: Mutex<IpInfoModule>,
}

impl AppModules {
    pub fn new() -> Self {
        Self {
            ip_info: Mutex::new(IpInfoModule::new()),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    DbError(sqlite::Error),
    InvalidOperation,
    InvalidInput,
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for Error {}
impl From<sqlite::Error> for Error {
    fn from(value: sqlite::Error) -> Self {
        Self::DbError(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
