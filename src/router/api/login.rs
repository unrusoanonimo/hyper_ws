use http::Response;
use hyper::Body;
use uuid::Uuid;

use crate::{
    model::user::User,
    util::{errors::OrServerError, AppError, XtendedResBuilder},
    ExtendedRequest, ModulesSendable,
};

use super::ROOT;

pub const PATH: &str = "/login";

const SUB: &str = "/sub";
pub async fn router(
    req: &mut ExtendedRequest,
    url: &str,
    _modules: ModulesSendable,
) -> Result<Response<Body>, AppError> {
    match (req.method.as_str(), url) {
        ("POST", ROOT) => Ok(Response::builder().body(Body::from("a"))?),
        ("GET", ROOT) => Response::builder()
            .json(&User::new(
                Uuid::new_v4(),
                "pepe@gmail.com",
                "Pepe",
                false,
                "abc123.",
            ))
            .or_svr_err(),
        _ => Err(AppError::NOT_FOUND),
    }
}
