use std::{
    borrow::Cow,
    collections::HashMap,
    net::SocketAddr,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use futures_util::TryStreamExt;
use http::{request::Parts, Request, Response};
use hyper::{server::conn::AddrStream, Body};
use tokio::sync::{Mutex, MutexGuard};

#[derive(Debug)]
pub struct PreparedResponse {
    pub builder: http::response::Builder,
    pub body: Body,
}
impl PreparedResponse {
    pub fn new(body: Body, builder: http::response::Builder) -> Self {
        Self { body, builder }
    }
    pub fn build(self) -> Result<Response<Body>, http::Error> {
        self.builder.body(self.body)
    }
}

#[derive(Debug)]

pub struct ExtendedReqXtraData {
    pub local_addr: SocketAddr,
    pub remote_addr: SocketAddr,
}
impl ExtendedReqXtraData {
    pub fn new(conn: &AddrStream) -> Self {
        Self {
            local_addr: conn.local_addr(),
            remote_addr: conn.remote_addr(),
        }
    }
}

#[derive(Debug)]
pub enum BodyType {
    Avaible(Option<Body>),
    Readen(Option<Vec<u8>>),
}
#[derive(Debug)]
pub struct ExtendedRequest {
    body: BodyType,
    parts: Parts,
    cookie_map: HashMap<String, String>,
    xtra: Arc<Mutex<ExtendedReqXtraData>>,
}
impl ExtendedRequest {
    pub fn new(inner: Request<Body>, xtra: Arc<Mutex<ExtendedReqXtraData>>) -> Self {
        let (parts, body) = inner.into_parts();

        let cookie_map: HashMap<String, String> = parts
            .headers
            .get("cookie")
            .map(|v| {
                let cookie = String::from_utf8_lossy(v.as_bytes());
                cookie
                    .split("; ")
                    .map(|v| {
                        let index = v.find("=").unwrap();
                        (String::from(&v[..index]), String::from(&v[index + 1..]))
                    })
                    .collect()
            })
            .unwrap_or_else(|| HashMap::new());
        Self {
            parts,
            body: BodyType::Avaible(Some(body)),
            cookie_map,
            xtra,
        }
    }
    pub fn get_cookie(&self, cookie: &str) -> Option<&str> {
        self.cookie_map.get(cookie).map(|v| v.deref())
    }
    pub fn get_header(&self, key: &str) -> Option<&[u8]> {
        self.headers.get(key).map(|v| v.as_bytes())
    }
    pub fn take_header(&mut self, key: &str) -> Option<http::HeaderValue> {
        self.headers.remove(key)
    }
    pub async fn xtra(&self) -> MutexGuard<'_, ExtendedReqXtraData> {
        self.xtra.lock().await
    }
    pub fn clean_url(&self) -> Cow<'_, str> {
        let path = self.uri.path();
        let url = url_escape::decode(path);

        if !url.ends_with('/') {
            return url;
        }
        match url {
            Cow::Owned(mut s) => {
                s.pop();
                Cow::Owned(s)
            }
            Cow::Borrowed(s) => {
                let mut chars = s.chars();
                chars.next_back();
                Cow::Borrowed(chars.as_str())
            }
        }
    }
    async fn init_body(&mut self) {
        if let BodyType::Avaible(body) = &mut self.body {
            let data = read_body(body.take().unwrap()).await.ok();
            self.body = BodyType::Readen(data);
        }
    }
    pub async fn read_body(&mut self) -> Option<&[u8]> {
        self.init_body().await;

        if let BodyType::Readen(data) = &self.body {
            Some(data.as_ref()?)
        } else {
            unreachable!("up there body should be Readen");
        }
    }
    pub async fn take_body(&mut self) -> Option<Vec<u8>> {
        self.init_body().await;

        if let BodyType::Readen(data) = &mut self.body {
            data.take()
        } else {
            unreachable!("up there body should be Readen");
        }
    }
}

impl Deref for ExtendedRequest {
    type Target = Parts;
    fn deref(&self) -> &Self::Target {
        &self.parts
    }
}
impl DerefMut for ExtendedRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parts
    }
}
async fn read_body(body: Body) -> Result<Vec<u8>, hyper::Error> {
    body.try_fold(Vec::new(), |mut data, chunk| async move {
        data.extend_from_slice(&chunk);
        Ok(data)
    })
    .await
}
