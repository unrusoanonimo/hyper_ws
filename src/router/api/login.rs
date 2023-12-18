use http::Response;
use hyper::Body;

use crate::{util::AppError, ExtendedRequest, ModulesSendable};

use super::ROOT;

pub const PATH: &str = "/login";

const SUB: &str = "/sub";
pub async fn router(
    req: &mut ExtendedRequest,
    url: &str,
    modules: ModulesSendable,
) -> Result<Response<Body>, AppError> {
    match (req.method.as_str(), url) {
        ("POST", ROOT) => Ok(Response::builder().body(Body::from("a"))?),

        _ => Err(AppError::NOT_FOUND),
    }
}
