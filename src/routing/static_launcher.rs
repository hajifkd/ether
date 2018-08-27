use routing::launcher::Launcher;
use std::path::PathBuf;

use futures::future::ok;
use futures::prelude::*;
use request::Request;
use response::*;
use url;

use tokio::codec::BytesCodec;
use tokio::codec::Decoder;
use tokio::fs;

/// This must be used with Tokio runtime.

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
    type Ret = Box<Future<Item = Response<ResponseBody>, Error = String> + Send>;

    #[cfg(target_arch = "wasm32")]
    fn launch<U>(&self, _request: &Request<U>, paths: &[&str]) -> Option<Self::Ret>
    where
        U: Stream<Item = Vec<u8>, Error = String>,
    {
        None
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn launch<U>(&self, _request: &Request<U>, paths: &[&str]) -> Option<Self::Ret>
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

        let tokio_file = fs::File::open(path);

        Some(Box::new(tokio_file.then(|f| {
            ok::<_, String>(match f {
                Ok(f) => {
                    let codec = BytesCodec::new();
                    let framed = codec.framed(f);
                    Response::new(ResponseBody::ByteStream(Box::new(
                        framed.map(|bm| bm.freeze()).map_err(|e| format!("{}", e)),
                    )))
                }
                Err(_) => Response::builder()
                    .status(::StatusCode::NOT_FOUND)
                    .body(ResponseBody::Immediate("Not Found".to_owned()))
                    .unwrap(),
            })
        })))
    }
}
