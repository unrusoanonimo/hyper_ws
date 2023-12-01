use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use http::Response;
use hyper::Body;

use crate::{
    model::ip_info::DataFromIp, modules::ip_info::Api, AppError, ExtendedRequest, ModulesSendable,
};

use super::{check_route, subroute_args, ROOT};

const SUB: &str = "/sub";
pub async fn router(
    mut req: ExtendedRequest,
    url: &str,
    modules: ModulesSendable<'_>,
) -> Result<Response<Body>, AppError> {
    match (req.method.as_str(), url) {
        ("GET", ROOT) => Ok(Response::builder()
            .body(Body::from(
                "a", // modules.db.lock().await.get_by_name("Alice").unwrap(),
            ))
            .unwrap()),
        (_, "/echo") => {
            let body_data = req.take_body().await.unwrap_or_else(|| vec![]);
            Ok(Response::builder().body(Body::from(body_data)).unwrap())
        }
        ("GET", "/ip_log") => {
            let ip = req.xtra().await.remote_addr.ip();

            let response = reqwest::get(format!("https://ipinfo.io/{}/json", ip))
                .await
                .or(Err(AppError::SERVER_ERROR))?
                .text()
                .await
                .or(Err(AppError::SERVER_ERROR))?;

            let info =
                serde_json::from_str::<DataFromIp>(&response).or(Err(AppError::BAD_REQUEST))?;
            modules.ip_info.lock().await.register_visit(info)?;
            let flags = modules.ip_info.lock().await.get_flags()?;
            Ok(Response::builder()
                
                .body(Body::from(
                    serde_json::to_string(&flags).or(Err(AppError::SERVER_ERROR))?,
                ))
                .unwrap())
        }
        _ if check_route(url, SUB) => {
            let a: Box<[_]> = subroute_args(url).collect();
            dbg!(a);
            Ok(Response::builder().body(Body::from("a")).unwrap())
        }
        ("POST", "/upload") => {
            let name_uncheked = subroute_args(url).next().ok_or(AppError::BAD_REQUEST)?;

            let name = Path::new(&name_uncheked)
                .file_name()
                .ok_or(AppError::BAD_REQUEST)?;
            let mut path = PathBuf::from("public/uploads");

            path.push(&name);
            let mut file = File::create(path).unwrap();
            file.write_all(req.read_body().await.unwrap()).unwrap();

            let mut url = "/uploads/".to_string();
            url += &name.to_string_lossy();

            Ok(Response::builder().body(Body::from(url)).unwrap())
        }

        _ => Err(AppError::NOT_FOUND),
    }
}
