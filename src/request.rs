use futures::prelude::*;
use uri;
use Method;

use http;
use http::header;

#[cfg(not(target_arch = "wasm32"))]
use hyper::Body;

#[cfg(target_arch = "wasm32")]
use futures;

pub struct Request {
    pub method: Method,
    pub uri: uri::Uri,
    pub body: BodyStream,
    pub headers: header::HeaderMap<header::HeaderValue>,
}

// inner always impl's Stream<Item=[&u8], Error=Box<Error + Send + Sync>>
#[cfg(target_arch = "wasm32")]
pub struct BodyStream {
    inner: futures::future::IntoStream<futures::future::FutureResult<Option<Vec<u8>>, ()>>,
}

#[cfg(not(target_arch = "wasm32"))]
pub struct BodyStream {
    inner: Body,
}

impl BodyStream {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn empty() -> BodyStream {
        BodyStream {
            inner: Body::empty(),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn empty() -> BodyStream {
        BodyStream {
            inner: futures::future::ok(|| None).into_stream(),
        }
    }
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
    fn poll(&mut self) -> Poll<Option<Vec<u8>>, String> {
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
    pub fn from_hyper_request(req: http::Request<Body>) -> Request {
        let (
            http::request::Parts {
                method,
                uri,
                headers,
                ..
            },
            body,
        ) = req.into_parts();

        Request {
            method,
            uri,
            headers,
            body: BodyStream { inner: body },
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn from_raw_values(method: Method, uri: uri::Uri, body: String) -> Request {
        Request {
            method,
            uri,
            headers: header::HeaderMap::new(),
            body: BodyStream {
                inner: futures::future::ok(move || Some(body.into())).into_stream(),
            },
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new_no_body(method: Method, uri: uri::Uri) -> Request {
        Request {
            method,
            uri,
            headers: header::HeaderMap::new(),
            body: BodyStream {
                inner: futures::future::ok(move || None).into_stream(),
            },
        }
    }
}
