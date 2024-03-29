use std::{
    collections::HashMap,
    io::{self, Read},
    path::PathBuf,
};

use crate::{
    util::{files::path_in_dir, get_extension, AppError, PreparedResponse},
    ExtendedRequest, ModulesSendable,
};

use hyper::{Body, Response};
use once_cell::sync::Lazy;
use std::fs::File;

mod api;

const ROOT: &str = "";
const MIME_DEFAULT: &str = "application/octet-stream";
pub static PUB_DIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("./public"));

pub fn public_path(req: &mut ExtendedRequest, url: &str) -> Option<Response<Body>> {
    let mut path = PUB_DIR.clone();
    if url.len() > 0 {
        path.push(&url[1..]);
    }
    if !path_in_dir(&PUB_DIR, &path) {
        return None;
    };
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
            get_mime(get_extension(url).unwrap_or_default()),
        )
        .body(prep.body)
        .ok()
}

pub fn get_mime(extension: &str) -> &'static str {
    *MIMEMAP.get(extension).unwrap_or(&MIME_DEFAULT)
}

static MIMEMAP: Lazy<HashMap<&str, &str>> =
    Lazy::new(|| serde_json::from_str(include_str!("../long_lines/mime.json")).unwrap());

pub fn send_file(mut file: File) -> Result<PreparedResponse, io::Error> {
    let mut data = vec![];
    let len = file.read_to_end(&mut data)?;
    let body = Body::from(data);
    let builder = Response::builder().header("Content-Length", len);
    Ok(PreparedResponse::new(body, builder))
}
pub fn redirect(path: &str, status: u16) -> Result<Response<Body>, AppError> {
    debug_assert!((300..400).contains(&status));

    Ok(Response::builder()
        .status(status)
        .header("Location", path)
        .body(Body::empty())?)
}

pub fn check_route(url: &str, route: &str) -> bool {
    url.starts_with(route) && (url.len() == route.len() || url.as_bytes()[route.len()] == b'/')
}
pub fn subroute_args(url: &str) -> std::str::Split<'_, char> {
    (&url[1..]).split('/')
}

#[allow(dead_code, unused_variables)]
async fn todo_router(
    req: ExtendedRequest,
    url: &str,
    modules: ModulesSendable,
) -> Result<Response<Body>, AppError> {
    panic!("todo_router is only inteded to be used as placeholder");
}

pub async fn main_router(
    req: &mut ExtendedRequest,
    modules: ModulesSendable,
) -> Result<Response<Body>, AppError> {
    let url: &str = &req.clean_url().to_string();

    let result = match (req.method.as_str(), url) {
        _ if check_route(url, "/a") => Ok(Response::builder().body(Body::from("value")).unwrap()),
        _ if check_route(url, api::PATH) => {
            api::router(req, &url[api::PATH.len()..], modules).await
        }
        ("GET", "/redirect") => redirect("/", 301),
        // ("POST", "/shell") => Response::builder()
        //     .json(&command::execute(String::from_utf8_lossy(
        //         req.read_body().await.unwrap(),
        //     )))
        //     .map_err(|_| AppError::SERVER_ERROR),
        ("GET", _) => public_path(req, url).ok_or(AppError::NOT_FOUND),
        _ => Err(AppError::NOT_FOUND),
    };
    result
}
