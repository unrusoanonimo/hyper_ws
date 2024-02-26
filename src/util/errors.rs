use hyper::{Body, Response};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::fs::File;

use crate::modules;
use crate::router::send_file;
use crate::util::XtendedResBuilder;

pub trait OrServerError<T> {
    fn or_svr_err(self) -> Result<T, AppError>;
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    status: u16,
    msg: String,
}

#[derive(Debug)]
pub enum AppError {
    StatusCode(u16),
    ApiError(ApiError),
    Dev(&'static str),
    Generic(String),
}
impl AppError {
    pub fn api_error(status: u16, msg: impl Into<String>) -> Self {
        Self::ApiError(ApiError {
            msg: msg.into(),
            status,
        })
    }
    pub const SERVER_ERROR: Self = AppError::StatusCode(500);
    pub const BAD_REQUEST: Self = AppError::StatusCode(400);
    pub const FORBIDDEN: Self = AppError::StatusCode(403);
    pub const NOT_FOUND: Self = AppError::StatusCode(404);
}
impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let details: Cow<'_, str> = match self {
            AppError::ApiError(v) => Cow::Owned(serde_json::to_string(v).unwrap()),
            AppError::StatusCode(s) => Cow::Owned(format!("status ({})", s)),
            AppError::Dev(v) => Cow::Borrowed(v),
            AppError::Generic(s) => Cow::Borrowed(s),
        };
        write!(f, "AppError: {:?}", details)
    }
}
impl TryInto<u16> for AppError {
    type Error = ();
    fn try_into(self) -> Result<u16, Self::Error> {
        match self {
            AppError::ApiError(e) => Ok(e.status),
            AppError::StatusCode(s) => Ok(s),
            AppError::Dev(_) | AppError::Generic(_) => Err(()),
        }
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

impl From<http::Error> for AppError {
    fn from(value: http::Error) -> Self {
        AppError::Generic(value.to_string())
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
            AppError::ApiError(e) => Response::builder().json(&e).unwrap(),
            AppError::Dev(msg) => {
                log::error!("{}", msg);
                AppError::SERVER_ERROR.into()
            }
            AppError::Generic(msg) => {
                log::error!("{}", msg);
                AppError::SERVER_ERROR.into()
            }
        }
    }
}

impl<T> OrServerError<T> for Option<T> {
    fn or_svr_err(self) -> Result<T, AppError> {
        self.ok_or(AppError::Dev("Unexpected None"))
    }
}
impl<T, E: Error> OrServerError<T> for Result<T, E> {
    fn or_svr_err(self) -> Result<T, AppError> {
        self.map_err(|e| AppError::Generic(e.to_string()))
    }
}
