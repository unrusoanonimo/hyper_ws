use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use http::Response;
use hyper::Body;

use crate::{AppError, ExtendedRequest};

use super::ROOT;

pub async fn router(mut req: ExtendedRequest, url: &str) -> Result<Response<Body>, AppError> {
    match (req.method.as_str(), url) {
        ("GET", ROOT) => Ok(Response::builder().body(Body::from("root")).unwrap()),
        (_, "/echo") => {
            let body_data = req.take_body().await.unwrap_or_else(|| vec![]);
            Ok(Response::builder().body(Body::from(body_data)).unwrap())
        }
        ("POST", "/upload") => {
            let name_header =
                String::from_utf8(req.get_header("File-Name").unwrap_or(&[]).to_vec()).unwrap();

            let extension = Path::new(&name_header)
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();
            let mut path = PathBuf::from("public/uploads");

            let file_content = req.read_body().await.unwrap();
            let mut file_name = sha256::digest(file_content);
            if extension.len() > 0 {
                file_name.push('.');
                file_name += extension;
            }

            path.push(&file_name);
            let mut file = File::create(path).unwrap();
            file.write_all(req.read_body().await.unwrap()).unwrap();

            let mut url = "/uploads/".to_string();
            url += &file_name;

            Ok(Response::builder().body(Body::from(url)).unwrap())
        }

        _ => Err(AppError::StatusCode(404)),
    }
}
