use std::fmt::Display;

use mysql::{OptsBuilder, Pool, PooledConn};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

pub mod fssa;
pub mod ip_info;
pub mod user;

pub use self::{fssa::FssaModule, ip_info::IpInfoModule, user::UserModule};

pub struct AppModules {
    pub ip_info: Mutex<IpInfoModule>,
    // pub user: UserModule,
    pub fssa: FssaModule,
}

static DB_POOL: Lazy<Pool> = Lazy::new(|| {
    let opts = OptsBuilder::new()
        .user(Some("atenda_prime"))
        .db_name(Some("atenda_prime"))
        .pass(Some("abc123."));
    Pool::new(opts).unwrap()
});

impl AppModules {
    pub fn new() -> Self {
        Self {
            ip_info: Mutex::new(IpInfoModule::new()),
            // user: UserModule::new(),
            fssa: FssaModule::new(),
        }
    }
    pub fn get_conncection() -> PooledConn {
        DB_POOL.get_conn().unwrap()
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
