use ::Method;
use ::mounter;

use std;

pub trait Launcher: std::marker::Sized {
    // TODO use some template engine so that we may use vdom
    // TODO not Method but something like Request, using Request.method instead.
    fn launch(&self, Method, &str) -> Option<String>;

    // TODO not to search '/' multiple times?
    // Maybe we can write mount! macro
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
            fn launch(&self, m: $crate::Method, path: &str) -> Option<String> {
                use std::vec::Vec;
                let path: Vec<_> = path.split('/').collect();
                $(
                    if let Some(a) = $route.match_route(m, &path[1..]) {
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
    #[test]
    fn test_empty() {
        use ::launcher::Launcher;
        let launcher = launcher!([]);

        assert_eq!(launcher.launch(::Method::Get, "/doc/hoge"), None);
    }

    #[test]
    fn test_simple_launcher() {
        use route::Route;
        use ::launcher::Launcher;

        let launcher = launcher!([ route!(::Method::Get; "hoge", "fuga") => || "piyo".to_owned() ]);

        assert_eq!(launcher.launch(::Method::Get, "/doc/hoge"), None);
        assert_eq!(launcher.launch(::Method::Post, "/hoge/fuga"), None);
        assert_eq!(
            launcher.launch(::Method::Get, "/hoge/fuga"),
            Some("piyo".to_owned())
        );
    }

    #[test]
    fn test_params() {
        use route::Route;
        use ::launcher::Launcher;

        let launcher =
            launcher!([ route!(::Method::Get; "hoge", String) => |x| format!("get {}", x) ]);

        assert_eq!(launcher.launch(::Method::Get, "/doc/hoge"), None);
        assert_eq!(
            launcher.launch(::Method::Get, "/hoge/fuga"),
            Some("get fuga".to_owned())
        );

        let launcher =
            launcher!([ route!(::Method::Get; "hoge", i32) => |x| format!("get {}", x + 5) ]);

        assert_eq!(launcher.launch(::Method::Get, "/doc/hoge"), None);
        assert_eq!(
            launcher.launch(::Method::Get, "/hoge/3"),
            Some("get 8".to_owned())
        );
    }

    #[test]
    fn test_multi_route() {
        use route::Route;
        use ::launcher::Launcher;

        let launcher = launcher!(
            [
                route!(::Method::Get; "hoge", "fuga") => || "no param /hoge/fuga".to_owned();
                route!(::Method::Get; "hoge", String) => |x: String| format!("param /hoge/{}", x);
                route!(::Method::Get; "piyo", i32) => |x: i32| format!("int param /piyo/{}", x);
            ]
        );

        assert_eq!(launcher.launch(::Method::Get, "/doc/hoge"), None);
        assert_eq!(
            launcher.launch(::Method::Get, "/hoge/fuga"),
            Some("no param /hoge/fuga".to_owned())
        );
        assert_eq!(
            launcher.launch(::Method::Get, "/hoge/fuga2"),
            Some("param /hoge/fuga2".to_owned())
        );
        assert_eq!(
            launcher.launch(::Method::Get, "/piyo/2"),
            Some("int param /piyo/2".to_owned())
        );
        assert_eq!(launcher.launch(::Method::Get, "/piyo/hoge"), None);
    }
}
