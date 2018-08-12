extern crate futures;
extern crate hyper;

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

pub trait Launcher: std::marker::Sized {
    // TODO use some template engine so that we may use vdom
    fn launch(&self, &Method, &str) -> Option<String>;

    // TODO not to search '/' multiple times?
    // Maybe we can write mount! macro
    fn mount<'a, S: Launcher>(self, prefix: &'a str, other: S) -> mounter::Mounter<'a, Self, S> {
        mounter::Mounter {
            without_prefix: self,
            prefix: prefix,
            with_prefix: other,
        }
    }
}

#[macro_use]
pub mod launcher_server;

pub mod mounter;

#[macro_use]
pub mod route;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
