use hyper::{Body, Response};
use std::error::Error;
use std::fmt::Display;
use std::fs::File;

use crate::modules;
use crate::router::send_file;

use super::unsafe_utils::Sendable;

#[derive(Debug)]
pub enum AppError {
    StatusCode(u16),
    Dev(&'static str),
    Generic(Sendable<Box<dyn Error>>),
}
impl AppError {
    pub const SERVER_ERROR: Self = AppError::StatusCode(500);
    pub const BAD_REQUEST: Self = AppError::StatusCode(400);
    pub const FORBIDDEN: Self = AppError::StatusCode(403);
    pub const NOT_FOUND: Self = AppError::StatusCode(404);
}
impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PETOU")
    }
}
impl Error for AppError {}
impl From<modules::Error> for AppError {
    fn from(value: modules::Error) -> Self {
        match value {
            modules::Error::DbError(_) => Self::SERVER_ERROR,
            modules::Error::InvalidInput => Self::BAD_REQUEST,
            modules::Error::InvalidOperation => Self::BAD_REQUEST,
        }
    }
}
impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::Generic(Sendable(Box::new(value)))
    }
}
impl From<http::Error> for AppError {
    fn from(value: http::Error) -> Self {
        AppError::Generic(Sendable(Box::new(value)))
    }
}
impl From<Box<dyn Error>> for AppError {
    fn from(value: Box<dyn Error>) -> Self {
        AppError::Generic(Sendable(value))
    }
}
impl From<reqwest::Error> for AppError {
    fn from(value: reqwest::Error) -> Self {
        AppError::Generic(Sendable(Box::new(value)))
    }
}
impl Into<Response<Body>> for AppError {
    fn into(self) -> Response<Body> {
        fn status_code(code: u16) -> Option<Response<Body>> {
            let file = File::open(format!("assets/error/{}.html", code)).ok()?;
            let prep = send_file(file).ok()?;
            prep.builder
                .header("Content-Type", "text/html")
                .body(prep.body)
                .ok()
        }
        match self {
            AppError::StatusCode(code) => status_code(code).unwrap_or(Response::new(Body::empty())),
            AppError::Dev(msg) => {
                log::error!("{}", msg);
                AppError::SERVER_ERROR.into()
            }
            AppError::Generic(e) => {
                log::error!("{:?}", &**e);
                AppError::SERVER_ERROR.into()
            }
        }
    }
}
