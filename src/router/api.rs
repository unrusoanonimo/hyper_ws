use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use http::Response;
use hyper::Body;

use crate::{model, AppError, ExtendedRequest, ModulesSendable};

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
                "a"
                // modules.db.lock().await.get_by_name("Alice").unwrap(),
            ))
            .unwrap()),
        (_, "/echo") => {
            let body_data = req.take_body().await.unwrap_or_else(|| vec![]);
            Ok(Response::builder().body(Body::from(body_data)).unwrap())
        }
        ("GET", "/count") => {
            let mut count = modules.c.lock().await;
            *count += 1;
            let visites = *count;
            drop(count);

            let ip = req.xtra().await.remote_addr.ip();

            dbg!(ip);
            let response = reqwest::get(format!("https://ipinfo.io/{}/json", ip))
                .await
                .or(Err(AppError::SERVER_ERROR))?
                .text()
                .await
                .or(Err(AppError::SERVER_ERROR))?;

            let r = serde_json::from_str::<model::IpInfo>(&response).ok();
            println!("{}", response);
            dbg!(r);

            Ok(Response::builder()
                .body(Body::from(format!("TOTAL_VISITES={visites}")))
                .unwrap())
        }
        _ if check_route(url, SUB) => {
            let a: Box<[_]> = subroute_args(url).collect();
            dbg!(a);
            Ok(Response::builder().body(Body::from("a")).unwrap())
        }
        ("POST", "/upload") => {
            let name_uncheked = subroute_args(url).next().ok_or(AppError::StatusCode(400))?;

            let name = Path::new(&name_uncheked)
                .file_name()
                .ok_or(AppError::StatusCode(400))?;
            let mut path = PathBuf::from("public/uploads");

            path.push(&name);
            let mut file = File::create(path).unwrap();
            file.write_all(req.read_body().await.unwrap()).unwrap();

            let mut url = "/uploads/".to_string();
            url += &name.to_string_lossy();

            Ok(Response::builder().body(Body::from(url)).unwrap())
        }

        _ => Err(AppError::StatusCode(404)),
    }
}
