#![allow(dead_code)]
use config::CONFIG;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Response, Server};
use log::info;
use modules::AppModules;
use prerouting_modules::PreroutingModules;
use zip::write::FileOptions;
use std::convert::Infallible;
use std::fs;
use std::io::Cursor;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use util::zip_utils::add_dir;
use util::{ExtendedReqXtraData, ExtendedRequest};
use zip::ZipWriter;

mod config;
mod init;
mod logger;
mod model;
mod modules;
mod prerouting_modules;
mod router;
mod util;

async fn handle(
    mut req: ExtendedRequest,
    modules: ModulesSendable,
    prerouting: PreroutingSendable,
) -> Result<Response<Body>, Infallible> {
    // dbg!(&req.uri, &req.clean_url());
    let result: Result<Response<Body>, util::AppError> =
        router::main_router(&mut req, modules.clone()).await;

    let response = match result {
        Ok(mut res) => {
            prerouting.modifiers.call(&mut req, &mut res, modules);
            res
        }
        Err(e) => {
            match e {
                util::AppError::StatusCode(_) => {}
                _ => log::error!("{}", e),
            }
            e.into()
        }
    };
    Ok(response)
}

type ModulesSendable = Arc<AppModules>;
type PreroutingSendable = Arc<PreroutingModules>;

#[tokio::main]
async fn main() {
    init::all();
    test().await;

    let modules: ModulesSendable = Arc::new(AppModules::new());
    let prerouting: PreroutingSendable = Arc::new(PreroutingModules::default());

    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from(([0, 0, 0, 0], CONFIG.port()));

    info!("server open at 127.0.0.1:{}", CONFIG.port());
    // And a MakeService to handle each connection...
    let make_service = make_service_fn(|conn: &AddrStream| {
        let xtra = Arc::new(Mutex::new(ExtendedReqXtraData::new(conn)));
        let module = modules.clone();
        let prerouting = prerouting.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle(
                    ExtendedRequest::new(req, xtra.clone()),
                    Arc::clone(&module),
                    Arc::clone(&prerouting),
                )
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
async fn test() {
    let cursor: Cursor<Vec<u8>> = Cursor::new(vec![]);

    let mut zip = ZipWriter::new(cursor);
    add_dir(
        &mut zip,
        "./data/fssa/client",
        "cli",
        FileOptions::default().compression_level(Some(9)),
    ).unwrap();
    fs::write("test.zip", zip.finish().unwrap().into_inner()).unwrap();
    // let modules: ModulesSendable = Arc::new(AppModules::new());
    // let r = modules.fssa.release().unwrap();
    // fs::write("test.zip", r).unwrap();
    // modules.user.test();
}
