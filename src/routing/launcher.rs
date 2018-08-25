use request::Request;
use response::Response;
use response::ResponseBody;
use routing::mounter;

use futures::prelude::*;

pub trait Launcher {
    // TODO use some template engine so that we may use vdom
    fn launch<T>(&self, request: &Request<T>, paths: &[&str]) -> Option<Response<ResponseBody>>
    where
        T: Stream<Item = Vec<u8>, Error = String>;

    fn mount<'a, S>(self, prefix: &'a str, other: S) -> mounter::Mounter<'a, Self, S>
    where
        Self: Sized,
        S: Launcher,
    {
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

        use $crate::_futures::*;

        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        struct __Ether_Launcher;

        impl $crate::routing::launcher::Launcher for __Ether_Launcher {
            #[allow(unused_variables)]
            fn launch<T>(&self, request: &$crate::request::Request<T>, paths: &[&str]) ->  Option<Response<ResponseBody>>
            where T: Stream<Item = Vec<u8>, Error = String> {
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
    use futures::prelude::*;
    use request::*;
    use response::*;
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

    fn responsify<T: Stream<Item = Vec<u8>, Error = String>>(
        f: impl Fn(&Request<T>) -> String,
    ) -> impl Fn(&Request<T>) -> Response<ResponseBody> {
        move |r| {
            let res = f(r);
            Response::new(ResponseBody::Immediate(res))
        }
    }

    fn responsify2<S: Stream<Item = Vec<u8>, Error = String>, T>(
        f: impl Fn(&Request<S>, T) -> String,
    ) -> impl Fn(&Request<S>, T) -> Response<ResponseBody> {
        move |s, t| {
            let res = f(s, t);
            Response::new(ResponseBody::Immediate(res))
        }
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
    fn test_empty() {
        let launcher = launcher!([]);

        assert_immediate!(
            launcher.launch(&empty_req!(::Method::GET), &routify("/doc/hoge")),
            None
        );
    }

    #[test]
    fn test_simple_launcher() {
        let launcher = launcher!([ route!(::Method::GET; "hoge", "fuga") => responsify(|_| "piyo".to_owned()) ]);

        assert_immediate!(
            launcher.launch(&empty_req!(::Method::GET), &routify("/doc/hoge")),
            None
        );
        assert_immediate!(
            launcher.launch(&empty_req!(::Method::POST), &routify("/hoge/fuga")),
            None
        );
        assert_immediate!(
            launcher.launch(&empty_req!(::Method::GET), &routify("/hoge/fuga")),
            Some("piyo".to_owned())
        );
    }

    #[test]
    fn test_params() {
        let launcher = launcher!([ route!(::Method::GET; "hoge", String) => responsify2(|_, x| format!("get {}", x)) ]);

        assert_immediate!(
            launcher.launch(&empty_req!(::Method::GET), &routify("/doc/hoge")),
            None
        );
        assert_immediate!(
            launcher.launch(&empty_req!(::Method::GET), &routify("/hoge/fuga")),
            Some("get fuga".to_owned())
        );

        let launcher = launcher!([ route!(&::Method::GET; "hoge", i32) => responsify2(|_, x| format!("get {}", x + 5)) ]);

        assert_immediate!(
            launcher.launch(&empty_req!(::Method::GET), &routify("/doc/hoge")),
            None
        );
        assert_immediate!(
            launcher.launch(&empty_req!(::Method::GET), &routify("/hoge/3")),
            Some("get 8".to_owned())
        );
    }

    #[test]
    fn test_multi_route() {
        let launcher = launcher!(
            [
                route!(&::Method::GET; "hoge", "fuga") => responsify(|_| "no param /hoge/fuga".to_owned());
                route!(&::Method::GET; "hoge", String) => responsify2(|_, x: String| format!("param /hoge/{}", x));
                route!(&::Method::GET; "piyo", i32) => responsify2(|_, x: i32| format!("int param /piyo/{}", x));
            ]
        );

        assert_immediate!(
            launcher.launch(&empty_req!(::Method::GET), &routify("/doc/hoge")),
            None
        );
        assert_immediate!(
            launcher.launch(&empty_req!(::Method::GET), &routify("/hoge/fuga")),
            Some("no param /hoge/fuga".to_owned())
        );
        assert_immediate!(
            launcher.launch(&empty_req!(::Method::GET), &routify("/hoge/fuga2")),
            Some("param /hoge/fuga2".to_owned())
        );
        assert_immediate!(
            launcher.launch(&empty_req!(::Method::GET), &routify("/piyo/2")),
            Some("int param /piyo/2".to_owned())
        );
        assert_immediate!(
            launcher.launch(&empty_req!(::Method::GET), &routify("/piyo/hoge")),
            None
        );
    }
}
