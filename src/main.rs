#![allow(dead_code)]
use config::CONFIG;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Response, Server};
use log::info;
use modules::AppModules;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use util::{ExtendedReqXtraData, ExtendedRequest};

mod config;
mod logger;
mod model;
mod modules;
mod router;
mod util;

async fn handle(
    req: ExtendedRequest,
    modules: ModulesSendable,
) -> Result<Response<Body>, Infallible> {
    let res = router::main_router(req, modules).await;
    Ok(match res {
        Ok(r) => r,
        Err(e) => {
            log::error!("{}", e);
            e.into()
        }
    })
}

type ModulesSendable = Arc<AppModules>;

#[tokio::main]
async fn main() {
    logger::setup();

    let modules: ModulesSendable = Arc::new(AppModules::new());
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
