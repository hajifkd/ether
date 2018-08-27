use futures::prelude::*;
use uri;
use Method;

use http;
use http::header;

#[cfg(not(target_arch = "wasm32"))]
use hyper::Body;

use std::cell::RefCell;

use futures;

pub struct Request<T> {
    pub method: Method,
    pub uri: uri::Uri,
    body: RefCell<Option<T>>,
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
        body: RefCell::new(Some(body.map(|c| c.to_vec()).map_err(|e| format!("{}", e)))),
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
        body: RefCell::new(Some(
            futures::future::ok(body.into())
                .into_stream()
                .map_err(|()| "Some error in future".to_owned()),
        )),
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
        body: RefCell::new(None),
    }
}

impl<T, V, E> Request<T>
where
    T: Stream<Item = V, Error = E>,
    V: Default,
{
    /// Take the body stream.
    /// Return `None` if it is already taken by either `take_stream` or `take_as_future`.
    pub fn take_stream(&self) -> Option<impl Stream<Item = V, Error = E>> {
        self.body.replace(None).take()
    }

    /// Take the body stream converted into future.
    /// Return `None` if it is already taken by either `take_stream` or `take_as_future`.
    pub fn take_as_future(&self) -> Option<impl Future<Item = V, Error = E>> {
        self.body.replace(None).take().map(|s| {
            s.into_future()
                .map(|(r, _)| {
                    if r.is_some() {
                        r.unwrap()
                    } else {
                        V::default()
                    }
                })
                .map_err(|(e, _)| e)
        })
    }
}
