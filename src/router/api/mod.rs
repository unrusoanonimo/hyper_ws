use http::Response;
use hyper::Body;

use crate::{
    util::{AppError, XtendedResBuilder},
    ExtendedRequest, ModulesSendable,
};

mod example;
mod login;
mod fssa;

use super::{check_route, subroute_args, ROOT};

pub const PATH: &str = "/api";

const SUB: &str = "/sub";
pub async fn router(
    req: &mut ExtendedRequest,
    url: &str,
    modules: ModulesSendable,
) -> Result<Response<Body>, AppError> {
    match (req.method.as_str(), url) {
        _ if check_route(url, login::PATH) => {
            login::router(req, &url[login::PATH.len()..], modules).await
        },
        _ if check_route(url, fssa::PATH) => {
            fssa::router(req, &url[fssa::PATH.len()..], modules).await
        }
        (_, ROOT) => Ok(Response::builder().text("api root")),
        _ => Err(AppError::NOT_FOUND),
    }
}
