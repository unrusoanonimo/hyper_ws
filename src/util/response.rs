use std::fs::File;

use http::{response, Response};
use hyper::Body;
use serde::Serialize;

use crate::router::get_mime;

use super::get_extension;

pub trait XtendedResBuilder {
    fn json<T: Serialize>(self, v: &T) -> Result<Response<Body>, serde_json::Error>;
    fn text(self, v: &str) -> Response<Body>;
    fn raw_data(self, d: impl Into<Vec<u8>>) -> Response<Body>;
    fn file(self, filename: &str, data: impl Into<Vec<u8>>) -> Response<Body>;
}
impl XtendedResBuilder for response::Builder {
    fn raw_data(self, d: impl Into<Vec<u8>>) -> Response<Body> {
        let data = d.into();
        self.header("content-length", data.len())
            .body(Body::from(data))
            .unwrap()
    }
    fn json<T: Serialize>(self, v: &T) -> Result<Response<Body>, serde_json::Error> {
        let data = serde_json::to_vec(v)?;
        Ok(self
            .header("Content-Type", "application/json")
            .raw_data(data))
    }
    fn text(self, v: &str) -> Response<Body> {
        self.header("Content-Type", "text/plain").raw_data(v)
    }
    fn file(self, filename: &str, data: impl Into<Vec<u8>>) -> Response<Body> {
        let mime = get_mime(get_extension(filename).unwrap_or_default());
        self.header("Content-Type", mime).header("Content-Disposition", format!("inline; filename=\"{filename}\"")).raw_data(data)
    }
}
