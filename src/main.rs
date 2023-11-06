#![allow(dead_code)]
use config::CONFIG;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Response, Server};
use log::info;
use modules::AppModules;
use rand::RngCore;
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
mod model;
mod modules;
mod router;
mod util;

#[derive(Debug)]
pub enum AppError {
    StatusCode(u16),
    Dev(&'static str),
}
impl AppError {
    pub const SERVER_ERROR: Self = AppError::StatusCode(500);
}
impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PETOU")
    }
}
impl Error for AppError {}

async fn handle(
    req: ExtendedRequest,
    modules: ModulesSendable<'_>,
) -> Result<Response<Body>, Infallible> {
    fn status_code(code: u16) -> Option<Response<Body>> {
        let file = File::open(format!("assets/error/{}.html", code)).ok()?;
        let prep = send_file(file).ok()?;
        prep.builder
            .header("Content-Type", "text/html")
            .body(prep.body)
            .ok()
    }
    let res = router::main_router(req, modules).await;
    Ok(match res {
        Ok(r) => r,
        Err(e) => match e {
            AppError::StatusCode(code) => status_code(code).unwrap_or(Response::new(Body::empty())),
            AppError::Dev(msg) => panic!("{}", msg),
        },
    })
}

type ModulesSendable<'a> = Arc<AppModules<'a>>;

#[tokio::main]
async fn main() {
    // use rand::Rng;
    let mut a: [u8; 16] = [0; 16];
    rand::thread_rng().fill_bytes(&mut a);

    logger::setup();

    let modules: ModulesSendable<'_> = Arc::new(AppModules::new());

    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from(([0, 0, 0, 0], CONFIG.port()));

    info!("server open at 127.0.0.1:{}", CONFIG.port());
    // And a MakeService to handle each connection...
    let make_service = make_service_fn(|conn: &AddrStream| {
        let xtra = Arc::new(Mutex::new(ExtendedReqXtraData::new(conn)));
        let module = modules.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle(ExtendedRequest::new(req, xtra.clone()), Arc::clone(&module))
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
