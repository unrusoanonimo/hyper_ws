use std::fmt::Display;

use tokio::sync::Mutex;

mod ip_info;
pub use ip_info::IpInfoModule;

pub struct AppModules<'a> {
    pub ip_info: Mutex<IpInfoModule<'a>>,
}
unsafe impl<'a> Sync for AppModules<'a> {}

impl<'a> AppModules<'a> {
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
