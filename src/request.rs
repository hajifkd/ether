use futures::prelude::*;
use Method;

use http;
use http::header;

#[cfg(not(target_arch = "wasm32"))]
use hyper::Body;

pub struct Request {
    pub method: Method,
    pub uri: ::uri::Uri,
    pub body: BodyStream,
    #[cfg(target_arch = "wasm32")]
    pub headers: (), // TODO ok?
    #[cfg(not(target_arch = "wasm32"))]
    pub headers: header::HeaderMap<header::HeaderValue>,
}

// inner always impl's Stream<Item=[&u8], Error=Box<Error + Send + Sync>>
#[cfg(target_arch = "wasm32")]
pub struct BodyStream {
    inner: futures::future::IntoStream<futures::future::FutureResult<Option<[&u8]>, ()>>,
}

#[cfg(not(target_arch = "wasm32"))]
pub struct BodyStream {
    inner: Body,
}

impl Stream for BodyStream {
    type Item = Vec<u8>;
    type Error = String;

    #[cfg(not(target_arch = "wasm32"))]
    fn poll(&mut self) -> Poll<Option<Vec<u8>>, String> {
        match self.inner.poll() {
            Err(e) => Err(format!("{}", e)),
            Ok(Async::Ready(Some(chunk))) => Ok(Async::Ready(Some(chunk.to_vec()))),
            Ok(Async::Ready(None)) => Ok(Async::Ready(None)),
            Ok(Async::NotReady) => Ok(Async::NotReady),
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn poll(&mut self) -> Poll<Option<[&u8]>, String> {
        match inner.poll() {
            Err(e) => "Error".to_owned(),
            Ok(Async::Ready(Some(Some(b)))) => Ok(Async::Ready(Some(b))),
            Ok(Async::Ready(Some(None))) => Ok(Async::Ready(None)),
            Ok(Async::Ready(None)) => Ok(Async::Ready(None)),
            Ok(Async::NotReady) => Ok(Async::NotReady), // Never reach here
        }
    }
}

impl Request {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_hyper_request(req: http::Request<Body>) {
        let (
            http::request::Parts {
                method,
                uri,
                headers,
                ..
            },
            body,
        ) = req.into_parts();
    }
}
