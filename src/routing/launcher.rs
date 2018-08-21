use request::Request;
use routing::mounter;

use futures::prelude::*;

use std;

pub trait Launcher<T: Stream<Item = Vec<u8>, Error = String>>: std::marker::Sized {
    // TODO use some template engine so that we may use vdom
    // TODO not Method but something like Request, using Request.method, .path instead.
    // take Option<Request>? Option::take looks ok.
    fn launch(&self, request: &mut Request<T>, paths: &[&str]) -> Option<String>;

    fn mount<'a, S>(self, prefix: &'a str, other: S) -> mounter::Mounter<'a, Self, S, T>
    where
        S: Launcher<T>,
    {
        mounter::Mounter {
            without_prefix: self,
            prefix: prefix,
            with_prefix: other,
            _d: std::marker::PhantomData,
        }
    }
}

#[macro_export]
macro_rules! launcher {
    ([ $( $route:expr => $fn:expr ;)+ ]) => (launcher!([ $( $route => $fn );* ]));

    ([ $( $route:expr => $fn:expr );* ]) => {{
        #[allow(unused_imports)]
        use $crate::utils::apply;

        use $crate::_futures::*;

        #[allow(non_camel_case_types)]
        struct __Ether_Launcher;

        impl<T: Stream<Item = Vec<u8>, Error = String>> $crate::routing::launcher::Launcher<T> for __Ether_Launcher {
            #[allow(unused_variables)]
            fn launch(&self, request: &mut $crate::request::Request<T>, paths: &[&str]) -> Option<String> {
                $(
                    // Never panic by this Option::unwrap.
                    if let Some(a) = $route.match_route(&request.method, paths) {
                        return Some(apply($fn, request, a));
                    }
                );*

                return None;
            }
        }

        __Ether_Launcher
    }}
}

#[cfg(test)]
mod tests {
    use routing::launcher::Launcher;
    use routing::route::Route;

    macro_rules! empty_req {
        ($m:expr) => {{
            use request;
            use uri;

            use std::str::FromStr;

            request::empty_body($m, uri::Uri::from_str("http://www.example.com/").unwrap())
        }};
    }

    fn routify<'a>(path: &'a str) -> Vec<&'a str> {
        path.split('/').skip(1).collect::<Vec<_>>()
    }

    #[test]
    fn test_empty() {
        let launcher = launcher!([]);

        assert_eq!(
            launcher.launch(&mut empty_req!(::Method::GET), &routify("/doc/hoge")),
            None
        );
    }

    #[test]
    fn test_simple_launcher() {
        let launcher =
            launcher!([ route!(::Method::GET; "hoge", "fuga") => |_| "piyo".to_owned() ]);

        assert_eq!(
            launcher.launch(&mut empty_req!(::Method::GET), &routify("/doc/hoge")),
            None
        );
        assert_eq!(
            launcher.launch(&mut empty_req!(::Method::POST), &routify("/hoge/fuga")),
            None
        );
        assert_eq!(
            launcher.launch(&mut empty_req!(::Method::GET), &routify("/hoge/fuga")),
            Some("piyo".to_owned())
        );
    }

    #[test]
    fn test_params() {
        let launcher =
            launcher!([ route!(::Method::GET; "hoge", String) => |_, x| format!("get {}", x) ]);

        assert_eq!(
            launcher.launch(&mut empty_req!(::Method::GET), &routify("/doc/hoge")),
            None
        );
        assert_eq!(
            launcher.launch(&mut empty_req!(::Method::GET), &routify("/hoge/fuga")),
            Some("get fuga".to_owned())
        );

        let launcher =
            launcher!([ route!(&::Method::GET; "hoge", i32) => |_, x| format!("get {}", x + 5) ]);

        assert_eq!(
            launcher.launch(&mut empty_req!(::Method::GET), &routify("/doc/hoge")),
            None
        );
        assert_eq!(
            launcher.launch(&mut empty_req!(::Method::GET), &routify("/hoge/3")),
            Some("get 8".to_owned())
        );
    }

    #[test]
    fn test_multi_route() {
        let launcher = launcher!(
            [
                route!(&::Method::GET; "hoge", "fuga") => |_| "no param /hoge/fuga".to_owned();
                route!(&::Method::GET; "hoge", String) => |_, x: String| format!("param /hoge/{}", x);
                route!(&::Method::GET; "piyo", i32) => |_, x: i32| format!("int param /piyo/{}", x);
            ]
        );

        assert_eq!(
            launcher.launch(&mut empty_req!(::Method::GET), &routify("/doc/hoge")),
            None
        );
        assert_eq!(
            launcher.launch(&mut empty_req!(::Method::GET), &routify("/hoge/fuga")),
            Some("no param /hoge/fuga".to_owned())
        );
        assert_eq!(
            launcher.launch(&mut empty_req!(::Method::GET), &routify("/hoge/fuga2")),
            Some("param /hoge/fuga2".to_owned())
        );
        assert_eq!(
            launcher.launch(&mut empty_req!(::Method::GET), &routify("/piyo/2")),
            Some("int param /piyo/2".to_owned())
        );
        assert_eq!(
            launcher.launch(&mut empty_req!(::Method::GET), &routify("/piyo/hoge")),
            None
        );
    }
}
