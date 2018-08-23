use futures::prelude::*;
use uri;
use Method;

use http;
use http::header;

#[cfg(not(target_arch = "wasm32"))]
use hyper::Body;

use futures;

pub struct Request<T: Stream<Item = Vec<u8>, Error = String>> {
    pub method: Method,
    pub uri: uri::Uri,
    body: Option<T>,
    pub headers: header::HeaderMap<header::HeaderValue>,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn from_hyper_request(
    req: http::Request<Body>,
) -> Request<impl Stream<Item = Vec<u8>, Error = String>> {
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
        body: Some(body.map(|c| c.to_vec()).map_err(|e| format!("{}", e))),
    }
}

pub fn from_raw_values(
    method: Method,
    uri: uri::Uri,
    body: String,
) -> Request<impl Stream<Item = Vec<u8>, Error = String>> {
    Request {
        method,
        uri,
        headers: header::HeaderMap::new(),
        body: Some(
            futures::future::ok(body.into())
                .into_stream()
                .map_err(|()| "Some error in future".to_owned()),
        ),
    }
}

pub fn empty_body(
    method: Method,
    uri: uri::Uri,
) -> Request<futures::stream::Empty<Vec<u8>, String>> {
    Request {
        method,
        uri,
        headers: header::HeaderMap::new(),
        body: None,
    }
}

impl<T: Stream<Item = Vec<u8>, Error = String>> Request<T> {
    /// Take the body stream.
    /// Return `None` if it is already taken by either `take_stream` or `take_as_future`.
    pub fn take_stream(&mut self) -> Option<impl Stream<Item = Vec<u8>, Error = String>> {
        self.body.take()
    }

    /// Take the body stream converted into future.
    /// Return `None` if it is already taken by either `take_stream` or `take_as_future`.
    pub fn take_as_future(&mut self) -> Option<impl Future<Item = Vec<u8>, Error = String>> {
        self.body.take().map(|s| {
            s.into_future()
                .map(|(r, _)| {
                    if r.is_some() {
                        r.unwrap()
                    } else {
                        vec![]
                    }
                })
                .map_err(|(e, _)| e)
        })
    }
}
