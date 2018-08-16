extern crate futures;
extern crate hyper;

#[macro_use]
pub mod route;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Method {
    Get,
    Post,
    Unknown,
}

impl std::convert::From<hyper::Method> for Method {
    fn from(method: hyper::Method) -> Self {
        match method {
            hyper::Method::GET => Method::Get,
            hyper::Method::POST => Method::Post,
            _ => Method::Unknown,
        }
    }
}

#[macro_use]
pub mod launcher;

pub mod mounter;

pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
