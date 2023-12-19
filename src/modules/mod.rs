use std::fmt::Display;

use sqlite::Connection;
use tokio::sync::Mutex;

pub mod ip_info;
pub mod user;

pub use ip_info::IpInfoModule;

use self::user::UserModule;

pub struct AppModules {
    pub ip_info: Mutex<IpInfoModule>,
    pub user: UserModule,
}

impl AppModules {
    pub fn new() -> Self {
        Self {
            ip_info: Mutex::new(IpInfoModule::new()),
            user: UserModule::new(),
        }
    }
    pub fn atenda_conection() -> Connection {
        Connection::open("data/atenda.sqlite").unwrap()
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
