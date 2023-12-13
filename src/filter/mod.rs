use std::sync::RwLock;

use http::Response;
use hyper::Body;
use once_cell::sync::Lazy;

use crate::{
    util::{AppError, ExtendedRequest},
    ModulesSendable,
};
pub enum FilterAction {
    None,
    Return(Response<Body>),
    Modify(Box<dyn FnOnce(&mut Response<Body>) -> Result<(), AppError>>),
}
pub trait HttpRequestFilter: Send + Sync {
    fn filter(
        &self,
        req: &ExtendedRequest,
        modules: &mut ModulesSendable,
    ) -> Result<FilterAction, AppError>;
}
pub static FILTERS: Lazy<RwLock<Vec<Box<dyn HttpRequestFilter>>>> =
    Lazy::new(|| RwLock::new(vec![]));
