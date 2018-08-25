use request::Request;
use routing::launcher::Launcher;

use futures::prelude::*;

use response::*;

pub struct Mounter<'a, S, T>
where
    S: Launcher,
    T: Launcher,
{
    pub(crate) without_prefix: S,
    pub(crate) prefix: &'a str,
    pub(crate) with_prefix: T,
}

impl<S, T> Clone for Mounter<'static, S, T>
where
    S: Launcher + Clone,
    T: Launcher + Clone,
{
    fn clone(&self) -> Self {
        Mounter {
            without_prefix: self.without_prefix.clone(),
            prefix: self.prefix,
            with_prefix: self.with_prefix.clone(),
        }
    }
}

impl<'a, S, T> Launcher for Mounter<'a, S, T>
where
    S: Launcher,
    T: Launcher,
{
    fn launch<U>(&self, request: &Request<U>, paths: &[&str]) -> Option<Response<ResponseBody>>
    where
        U: Stream<Item = Vec<u8>, Error = String>,
    {
        if let Some(r) = self.without_prefix.launch(request, paths) {
            Some(r)
        } else {
            if paths.len() > 0 && self.prefix == paths[0] {
                self.with_prefix.launch(request, &paths[1..])
            } else {
                None
            }
        }
    }
}
