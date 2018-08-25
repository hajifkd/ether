use futures::prelude::*;

use std;

pub use http::Response;

pub enum ResponseBody {
    Immediate(String),
    Stream(Box<Stream<Item = String, Error = String> + Send + 'static>),
}

impl std::convert::From<String> for ResponseBody {
    fn from(s: String) -> ResponseBody {
        ResponseBody::Immediate(s)
    }
}

#[cfg(not(target_arch = "wasm32"))]
use hyper::Body;

#[cfg(not(target_arch = "wasm32"))]
impl std::convert::Into<Body> for ResponseBody {
    fn into(self) -> Body {
        match self {
            ResponseBody::Immediate(s) => Body::from(s),
            ResponseBody::Stream(s) => Body::wrap_stream(s),
        }
    }
}
