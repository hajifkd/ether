use futures::future;
use futures::prelude::*;
use routing::launcher::*;
use std::net::SocketAddr;

use hyper;

use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server, StatusCode};

use request;

pub fn run<T>(addr: SocketAddr, launcher: T)
where
    T: Launcher + Send + Sync + Clone + 'static,
{
    let server = Server::bind(&addr)
        .serve(move || {
            let launcher = launcher.clone();
            service_fn(
                move |req: Request<Body>| -> Box<Future<Item = Response<Body>, Error = String> + Send> {
                    let mut request = request::from_hyper_request(req);
                    let path = request.uri.path().to_owned();
                    let paths = path.split('/').skip(1).collect::<Vec<_>>();
                    let result = launcher.launch(&mut request, &paths);

                    if let Some(r) = result {
                        Box::new(future::ok(Response::new(Body::from(r))))
                    } else {
                        let body = Body::from("Not found");
                        Box::new(future::ok(
                            Response::builder()
                                .status(StatusCode::NOT_FOUND)
                                .body(body)
                                .unwrap(),
                        ))
                    }
                },
            )
        })
        .map_err(|e| eprintln!("Error: {}", e));

    hyper::rt::run(server);
}
