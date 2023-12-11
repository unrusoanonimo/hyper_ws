use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use http::Response;
use hyper::Body;

use crate::{
    model::ip_info::DataFromIp,
    modules::ip_info::Api,
    util::{errors::OrServerError, AppError, XtendedResBuilder},
    ExtendedRequest, ModulesSendable,
};

use super::{check_route, subroute_args, ROOT};

const SUB: &str = "/sub";
pub async fn router(
    mut req: ExtendedRequest,
    url: &str,
    modules: ModulesSendable<'_>,
) -> Result<Response<Body>, AppError> {
    match (req.method.as_str(), url) {
        ("GET", ROOT) => Ok(Response::builder().body(Body::from("a"))?),
        (_, "/echo") => {
            let body_data = req.take_body().await.unwrap_or_else(|| vec![]);
            Ok(Response::builder().body(Body::from(body_data))?)
        }
        ("GET", "/ip_log") => {
            let ip = req.xtra().await.remote_addr.ip();

            let response = reqwest::get(format!("https://ipinfo.io/{}/json", ip))
                .await
                .or_svr_err()?
                .text()
                .await
                .or_svr_err()?;

            let info =
                serde_json::from_str::<DataFromIp>(&response).or(Err(AppError::BAD_REQUEST))?;
            modules.ip_info.lock().await.register_visit(info)?;

            let flags = modules.ip_info.lock().await.get_flags()?;
            Response::builder()
                .json(&flags)
                .or(Err(AppError::SERVER_ERROR))
        }
        _ if check_route(url, SUB) => {
            let a: Box<[_]> = subroute_args(url).collect();
            Response::builder()
                .body(Body::from(a.join(" ")))
                .or(Err(AppError::SERVER_ERROR))
        }

        ("POST", "/upload") => {
            let name_uncheked = subroute_args(url).next().ok_or(AppError::BAD_REQUEST)?;

            let name = Path::new(&name_uncheked)
                .file_name()
                .ok_or(AppError::BAD_REQUEST)?;
            let mut path = PathBuf::from("public/uploads");

            path.push(&name);
            let mut file = File::create(path).or_svr_err()?;
            file.write_all(req.read_body().await.ok_or(AppError::SERVER_ERROR)?)
                .or_svr_err()?;

            let mut url = "/uploads/".to_string();
            url += &name.to_string_lossy();

            Response::builder()
                .body(Body::from(url))
                .or(Err(AppError::SERVER_ERROR))
        }

        _ => Err(AppError::NOT_FOUND),
    }
}
