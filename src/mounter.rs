use launcher::Launcher;
use request::Request;

pub struct Mounter<'a, S, T>
where
    S: Launcher,
    T: Launcher,
{
    pub(crate) without_prefix: S,
    pub(crate) prefix: &'a str,
    pub(crate) with_prefix: T,
}

impl<'a, S, T> Launcher for Mounter<'a, S, T>
where
    S: Launcher,
    T: Launcher,
{
    fn launch(&self, request: &mut Option<Request>, paths: &[&str]) -> Option<String> {
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
