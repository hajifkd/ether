extern crate hyper;
extern crate futures;

use futures::{future, Future};
use hyper::{Server, Response, Body, Method, StatusCode, Request};
use hyper::service::service_fn;

const INDEX: &[u8] = b"hogehoge";
const NOT_FOUND: &[u8] = b"Not Found";

fn response(req: Request<Body>) -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send>
{
    println!("Request received: {}", req.uri());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let body = Body::from(INDEX);
            Box::new(future::ok(Response::new(body)))
        }

        _ => {
            let body = Body::from(NOT_FOUND);
            Box::new(future::ok(Response::builder()
                                         .status(StatusCode::NOT_FOUND)
                                         .body(body)
                                         .unwrap()))
        }
    }
}


fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    hyper::rt::run(future::lazy(move || {
        let new_service = || {
            service_fn(|req| {
                response(req)
            })
        };

        let server = Server::bind(&addr)
                            .serve(new_service)
                            .map_err(|e| eprintln!("Error: {}", e));

        println!("Listening on {}", addr);

        server
    }));
}
