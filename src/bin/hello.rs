#[macro_use]
extern crate ether;

extern crate futures;

use ether::request::Request;
use ether::response::*;
use ether::routing::route::Route;
use ether::Method;

fn index<T>(_r: &Request<T>) -> Response<ResponseBody> {
    Response::new(ResponseBody::Immediate("Hello, world!".to_owned()))
}

fn hello_with_name<T>(_r: &Request<T>, name: String) -> Response<ResponseBody> {
    Response::new(ResponseBody::Immediate(format!("Hello, {}!", name)))
}

fn main() {
    let launcher = launcher!([
        route!(&::Method::GET; "") => index;
        route!(&::Method::GET; "greeting", String) => hello_with_name;
    ]);

    ether::runtime::run("127.0.0.1:8080".parse().unwrap(), launcher);
}
