extern crate futures;
extern crate http;

#[cfg(not(target_arch = "wasm32"))]
extern crate hyper;

// TODO use http::Method itself?
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Method {
    Get,
    Post,
    Unknown,
}

impl std::convert::From<http::Method> for Method {
    fn from(method: http::Method) -> Self {
        match method {
            http::Method::GET => Method::Get,
            http::Method::POST => Method::Post,
            _ => Method::Unknown,
        }
    }
}

#[macro_use]
pub mod route;

#[macro_use]
pub mod launcher;

pub mod mounter;

pub mod utils;

pub mod request;

pub mod uri {
    pub use http::uri::Uri;
}
