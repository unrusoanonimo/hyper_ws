use std::{
    collections::HashMap,
    io::{self, Read},
    path::PathBuf,
};

use crate::{
    util::{get_extension, PreparedResponse},
    AppError, ExtendedRequest,
};

use hyper::{Body, Response};
use once_cell::sync::Lazy;
use std::fs::File;

mod api;

const ROOT: &str = "";
const MIME_DEFAULT: &str = "application/octet-stream";

pub fn public_path(req: ExtendedRequest, url: &str) -> Option<Response<Body>> {
    let mut inner_path = "./public".to_string();
    inner_path += url;
    let mut path = PathBuf::from(&inner_path);
    if path.is_dir() {
        if !req.uri.path().ends_with('/') {
            let mut new = url.to_owned();
            new.push('/');
            return Response::builder()
                .status(308)
                .header("Location", new)
                .body(Body::empty())
                .ok();
        }
        path.push("index.html");
        let file = File::open(path).ok()?;
        let prep = send_file(file).ok()?;
        return prep
            .builder
            .header("Content-Type", "text/html")
            .body(prep.body)
            .ok();
    }
    let file = File::open(path).ok()?;
    let prep = send_file(file).ok()?;
    prep.builder
        .header(
            "Content-Type",
            *MIMEMAP
                .get(get_extension(url).unwrap_or_default())
                .unwrap_or(&MIME_DEFAULT),
        )
        .body(prep.body)
        .ok()
}

static MIMEMAP: Lazy<HashMap<&str, &str>> = once_cell::sync::Lazy::new(|| {
    serde_json::from_str(include_str!("../long_lines/mime.json")).unwrap()
});

pub fn send_file(mut file: File) -> Result<PreparedResponse, io::Error> {
    let mut data = vec![];
    let len = file.read_to_end(&mut data)?;
    let body = Body::from(data);
    let builder = Response::builder().header("Content-Length", len);
    Ok(PreparedResponse::new(body, builder))
}
pub fn redirect(path: &str, status: u16) -> Response<Body> {
    debug_assert!((300..400).contains(&status));

    Response::builder()
        .status(status)
        .header("Location", path)
        .body(Body::empty())
        .unwrap()
}

#[allow(dead_code, unused_variables)]
async fn todo_router(req: ExtendedRequest, url: &str) -> Result<Response<Body>, AppError> {
    panic!("todo_router is only inteded to be used as placeholder");
}

const API: &str = "/api";
pub async fn main_router(req: ExtendedRequest) -> Result<Response<Body>, AppError> {
    let url: &str = &req.clean_url().to_string();
    match (req.method.as_str(), url) {
        _ if url.starts_with(API) => api::router(req, &url[API.len()..]).await,
        ("GET", "/redirect") => Ok(redirect("/", 301)),
        ("GET", _) => public_path(req, url).ok_or(AppError::StatusCode(404)),
        _ => Err(AppError::StatusCode(404)),
    }
}
