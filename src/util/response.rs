use http::{Response, response};
use hyper::Body;
use serde::Serialize;

pub trait XtendedResBuilder {
    fn json<T: Serialize>(self, v: &T) -> Result<Response<Body>, serde_json::Error>;
}
impl XtendedResBuilder for response::Builder {
    fn json<T: Serialize>(self, v: &T) -> Result<Response<Body>, serde_json::Error> {
        Ok(self
            .body(Body::from(serde_json::to_string(v)?))
            .unwrap())
    }
}
