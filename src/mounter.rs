use ::launcher::Launcher;

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
    fn launch(&self, method: ::Method, path: &str) -> Option<String> {
        if let Some(r) = self.without_prefix.launch(method, path) {
            Some(r)
        } else if path.len() > 0 {
            if let Some(at) = &path[1..].find('/') {
                let (fst, snd) = path.split_at(1usize + at);
                if fst == self.prefix {
                    self.with_prefix.launch(method, snd)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
