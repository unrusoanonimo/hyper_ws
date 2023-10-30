#![allow(dead_code)]
use config::CONFIG;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Response, Server};
use log::info;
use modules::AppModules;
use std::convert::Infallible;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use util::{ExtendedReqXtraData, ExtendedRequest};

use crate::router::send_file;

mod config;
mod logger;
mod modules;
mod router;
mod util;

#[derive(Debug)]
pub enum AppError {
    StatusCode(u16),
    Dev(&'static str),
}
impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PETOU")
    }
}
impl Error for AppError {}

async fn handle(req: ExtendedRequest) -> Result<Response<Body>, Infallible> {
    fn status_code(code: u16) -> Option<Response<Body>> {
        let file = File::open(format!("assets/error/{}.html", code)).ok()?;
        let prep = send_file(file).ok()?;
        prep.builder
            .header("Content-Type", "text/html")
            .body(prep.body)
            .ok()
    }
    let res = router::main_router(req).await;
    Ok(match res {
        Ok(r) => r,
        Err(e) => match e {
            AppError::StatusCode(code) => status_code(code).unwrap_or(Response::new(Body::empty())),
            AppError::Dev(msg) => panic!("{}", msg),
        },
    })
}

#[tokio::main]
async fn main() {
    logger::setup();

    let modules = Arc::new(Mutex::new(AppModules::new()));

    let mut m = modules.lock().await;
    dbg!(m.db.get_by_name("Alice"));
    dbg!(m.db.get_by_name("Alice"));
    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from(([0, 0, 0, 0], CONFIG.port()));

    info!("server open at 127.0.0.1:{}", CONFIG.port());
    // And a MakeService to handle each connection...
    let make_service = make_service_fn(|conn: &AddrStream| {
        let xtra = Arc::new(Mutex::new(ExtendedReqXtraData::new(conn)));
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle(ExtendedRequest::new(req, xtra.clone()))
            }))
        }
    });

    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
