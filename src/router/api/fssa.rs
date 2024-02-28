use http::Response;
use hyper::Body;

use crate::{
    modules::FssaModule,
    router::{check_route, subroute_args},
    util::{AppError, ExtendedRequest, XtendedResBuilder},
    ModulesSendable,
};

pub const PATH: &str = "/fssa";
pub async fn router(
    req: &mut ExtendedRequest,
    url: &str,
    modules: ModulesSendable,
) -> Result<Response<Body>, AppError> {
    match (req.method.as_str(), url) {
        ("GET", "/release") => {
            let data = modules.fssa.release().or(Err(AppError::SERVER_ERROR))?;
            Ok(Response::builder().file(FssaModule::MODPACK_FILENAME, data))
        }

        _ if check_route(url, "/mod") => {
            let args: Box<[_]> = subroute_args(url).collect();
            let filename = *args.get(0).ok_or(AppError::BAD_REQUEST)?;

            let data = modules.fssa.get_mod(filename).ok_or(AppError::api_error(
                AppError::NOT_FOUND.try_into().unwrap(),
                "mod does not exist",
            ))?;

            Ok(Response::builder().file(filename, data))
        }
        _ => Err(AppError::NOT_FOUND),
    }
}
