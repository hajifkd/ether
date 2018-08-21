extern crate futures;
extern crate http;

#[cfg(not(target_arch = "wasm32"))]
extern crate hyper;

#[macro_use]
pub mod routing;

pub mod utils;

pub mod request;

pub mod runtime;

pub mod uri {
    pub use http::uri::Uri;
}

pub use http::Method;

pub mod _futures {
    pub use futures::prelude::*;
}
