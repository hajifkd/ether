use request::Request;
use routing::launcher::Launcher;

use futures::prelude::*;

use std;

pub struct Mounter<'a, S, T, U>
where
    S: Launcher<U>,
    T: Launcher<U>,
    U: Stream<Item = Vec<u8>, Error = String>,
{
    pub(crate) without_prefix: S,
    pub(crate) prefix: &'a str,
    pub(crate) with_prefix: T,
    pub(crate) _d: std::marker::PhantomData<U>,
}

impl<'a, S, T, U> Launcher<U> for Mounter<'a, S, T, U>
where
    S: Launcher<U>,
    T: Launcher<U>,
    U: Stream<Item = Vec<u8>, Error = String>,
{
    fn launch(&self, request: &mut Request<U>, paths: &[&str]) -> Option<String> {
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
