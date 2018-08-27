use routing::launcher::Launcher;
use std::path::PathBuf;

use futures::prelude::*;
use request::Request;
use response::*;
use url;

use std;

#[derive(Clone)]
pub struct StaticLauncher {
    #[cfg(not(target_arch = "wasm32"))]
    path: PathBuf,
}

impl StaticLauncher {
    #[allow(unused_variables)]
    pub fn new(base: &str) -> StaticLauncher {
        #[cfg(not(target_arch = "wasm32"))]
        return StaticLauncher {
            path: PathBuf::from(base),
        };

        #[cfg(target_arch = "wasm32")]
        return StaticLauncher;
    }
}

impl Launcher for StaticLauncher {
    #[cfg(target_arch = "wasm32")]
    fn launch<U>(&self, _request: &Request<U>, paths: &[&str]) -> Option<Response<ResponseBody>>
    where
        U: Stream<Item = Vec<u8>, Error = String>,
    {
        None
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn launch<U>(&self, _request: &Request<U>, paths: &[&str]) -> Option<Response<ResponseBody>>
    where
        U: Stream<Item = Vec<u8>, Error = String>,
    {
        let path = {
            let mut path = self.path.clone();
            for p in paths {
                let p = url::percent_encoding::percent_decode(p.as_bytes()).decode_utf8_lossy();
                path.push(&*p);
            }
            path
        };

        if !path.starts_with(&self.path) {
            return None;
        }

        // TODO Async reading
        let result = std::fs::read_to_string(&path);
        if result.is_err() {
            return None;
        }

        Some(Response::new(ResponseBody::Immediate(result.unwrap())))
    }
}
