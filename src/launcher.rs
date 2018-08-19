use mounter;
use request::Request;

use std;

pub trait Launcher: std::marker::Sized {
    // TODO use some template engine so that we may use vdom
    // TODO not Method but something like Request, using Request.method, .path instead.
    // take Option<Request>? Option::take looks ok.
    fn launch(&self, request: &mut Option<Request>, paths: &[&str]) -> Option<String>;

    fn mount<'a, S: Launcher>(self, prefix: &'a str, other: S) -> mounter::Mounter<'a, Self, S> {
        mounter::Mounter {
            without_prefix: self,
            prefix: prefix,
            with_prefix: other,
        }
    }
}

#[macro_export]
macro_rules! launcher {
    ([ $( $route:expr => $fn:expr ;)+ ]) => (launcher!([ $( $route => $fn );* ]));

    ([ $( $route:expr => $fn:expr );* ]) => {{
        #[allow(unused_imports)]
        use $crate::utils::apply;

        #[allow(non_camel_case_types)]
        struct __Ether_Launcher;

        impl $crate::launcher::Launcher for __Ether_Launcher {
            #[allow(unused_variables)]
            fn launch(&self, request: &mut Option<$crate::request::Request>, paths: &[&str]) -> Option<String> {
                let ref m = request.as_ref().unwrap().method;

                $(
                    if let Some(a) = $route.match_route(m, paths) {
                        return Some(apply($fn, a));
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

    macro_rules! empty_req {
        ($m:expr) => {{
            use request::BodyStream;
            use request::Request;

            use http;
            use uri;

            use std::str::FromStr;

            Some(Request {
                method: $m,
                uri: uri::Uri::from_str("http://www.example.com/").unwrap(),
                body: BodyStream::empty(),
                headers: http::header::HeaderMap::new(),
            })
        }};
    }

    #[test]
    fn test_empty() {
        use launcher::Launcher;
        let launcher = launcher!([]);

        assert_eq!(
            launcher.launch(
                &mut empty_req!(::Method::GET),
                &"/doc/hoge".split('/').collect::<Vec<_>>()[1..]
            ),
            None
        );
    }

    #[test]
    fn test_simple_launcher() {
        use launcher::Launcher;
        use route::Route;

        let launcher = launcher!([ route!(::Method::GET; "hoge", "fuga") => || "piyo".to_owned() ]);

        assert_eq!(
            launcher.launch(
                &mut empty_req!(::Method::GET),
                &"/doc/hoge".split('/').collect::<Vec<_>>()[1..]
            ),
            None
        );
        assert_eq!(
            launcher.launch(
                &mut empty_req!(::Method::POST),
                &"/hoge/fuga".split('/').collect::<Vec<_>>()[1..]
            ),
            None
        );
        assert_eq!(
            launcher.launch(
                &mut empty_req!(::Method::GET),
                &"/hoge/fuga".split('/').collect::<Vec<_>>()[1..]
            ),
            Some("piyo".to_owned())
        );
    }

    #[test]
    fn test_params() {
        use launcher::Launcher;
        use route::Route;

        let launcher =
            launcher!([ route!(::Method::GET; "hoge", String) => |x| format!("get {}", x) ]);

        assert_eq!(
            launcher.launch(
                &mut empty_req!(::Method::GET),
                &"/doc/hoge".split('/').collect::<Vec<_>>()[1..]
            ),
            None
        );
        assert_eq!(
            launcher.launch(
                &mut empty_req!(::Method::GET),
                &"/hoge/fuga".split('/').collect::<Vec<_>>()[1..]
            ),
            Some("get fuga".to_owned())
        );

        let launcher =
            launcher!([ route!(&::Method::GET; "hoge", i32) => |x| format!("get {}", x + 5) ]);

        assert_eq!(
            launcher.launch(
                &mut empty_req!(::Method::GET),
                &"/doc/hoge".split('/').collect::<Vec<_>>()[1..]
            ),
            None
        );
        assert_eq!(
            launcher.launch(
                &mut empty_req!(::Method::GET),
                &"/hoge/3".split('/').collect::<Vec<_>>()[1..]
            ),
            Some("get 8".to_owned())
        );
    }

    #[test]
    fn test_multi_route() {
        use launcher::Launcher;
        use route::Route;

        let launcher = launcher!(
            [
                route!(&::Method::GET; "hoge", "fuga") => || "no param /hoge/fuga".to_owned();
                route!(&::Method::GET; "hoge", String) => |x: String| format!("param /hoge/{}", x);
                route!(&::Method::GET; "piyo", i32) => |x: i32| format!("int param /piyo/{}", x);
            ]
        );

        assert_eq!(
            launcher.launch(
                &mut empty_req!(::Method::GET),
                &"/doc/hoge".split('/').collect::<Vec<_>>()[1..]
            ),
            None
        );
        assert_eq!(
            launcher.launch(
                &mut empty_req!(::Method::GET),
                &"/hoge/fuga".split('/').collect::<Vec<_>>()[1..]
            ),
            Some("no param /hoge/fuga".to_owned())
        );
        assert_eq!(
            launcher.launch(
                &mut empty_req!(::Method::GET),
                &"/hoge/fuga2".split('/').collect::<Vec<_>>()[1..]
            ),
            Some("param /hoge/fuga2".to_owned())
        );
        assert_eq!(
            launcher.launch(
                &mut empty_req!(::Method::GET),
                &"/piyo/2".split('/').collect::<Vec<_>>()[1..]
            ),
            Some("int param /piyo/2".to_owned())
        );
        assert_eq!(
            launcher.launch(
                &mut empty_req!(::Method::GET),
                &"/piyo/hoge".split('/').collect::<Vec<_>>()[1..]
            ),
            None
        );
    }
}
