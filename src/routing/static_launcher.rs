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

#[cfg(test)]
mod test {
    use response::*;
    use routing::launcher::Launcher;
    use routing::static_launcher::StaticLauncher;

    macro_rules! empty_req {
        ($m:expr) => {{
            use request;
            use uri;

            use std::str::FromStr;

            request::empty_body($m, uri::Uri::from_str("http://www.example.com/").unwrap())
        }};
    }

    macro_rules! assert_immediate {
        ($a:expr, $b:expr) => {
            match $a {
                Some(r) => match r.into_body() {
                    ResponseBody::Immediate(s) => assert_eq!(Some(s), $b),
                    _ => panic!("Invalid body"),
                },
                None => assert_eq!(None::<String>, $b),
            }
        };
    }

    #[test]
    fn serve_static_file() {
        let launcher = StaticLauncher::new("./");
        assert_immediate!(
            launcher.launch(&empty_req!(::Method::GET), &["Cargo.toml"]),
            Some(include_str!("../../Cargo.toml").to_owned())
        );
    }
}
