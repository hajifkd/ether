use mounter;
use Method;

use std;

pub trait Launcher: std::marker::Sized {
    // TODO use some template engine so that we may use vdom
    // TODO not Method but something like Request, using Request.method, .path instead.
    fn launch(&self, &Method, &str) -> Option<String>;

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
            fn launch(&self, m: &$crate::Method, path: &str) -> Option<String> {
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
        use launcher::Launcher;
        let launcher = launcher!([]);

        assert_eq!(launcher.launch(&::Method::GET, "/doc/hoge"), None);
    }

    #[test]
    fn test_simple_launcher() {
        use launcher::Launcher;
        use route::Route;

        let launcher = launcher!([ route!(::Method::GET; "hoge", "fuga") => || "piyo".to_owned() ]);

        assert_eq!(launcher.launch(&::Method::GET, "/doc/hoge"), None);
        assert_eq!(launcher.launch(&::Method::POST, "/hoge/fuga"), None);
        assert_eq!(
            launcher.launch(&::Method::GET, "/hoge/fuga"),
            Some("piyo".to_owned())
        );
    }

    #[test]
    fn test_params() {
        use launcher::Launcher;
        use route::Route;

        let launcher =
            launcher!([ route!(::Method::GET; "hoge", String) => |x| format!("get {}", x) ]);

        assert_eq!(launcher.launch(&::Method::GET, "/doc/hoge"), None);
        assert_eq!(
            launcher.launch(&::Method::GET, "/hoge/fuga"),
            Some("get fuga".to_owned())
        );

        let launcher =
            launcher!([ route!(&::Method::GET; "hoge", i32) => |x| format!("get {}", x + 5) ]);

        assert_eq!(launcher.launch(&::Method::GET, "/doc/hoge"), None);
        assert_eq!(
            launcher.launch(&::Method::GET, "/hoge/3"),
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

        assert_eq!(launcher.launch(&::Method::GET, "/doc/hoge"), None);
        assert_eq!(
            launcher.launch(&::Method::GET, "/hoge/fuga"),
            Some("no param /hoge/fuga".to_owned())
        );
        assert_eq!(
            launcher.launch(&::Method::GET, "/hoge/fuga2"),
            Some("param /hoge/fuga2".to_owned())
        );
        assert_eq!(
            launcher.launch(&::Method::GET, "/piyo/2"),
            Some("int param /piyo/2".to_owned())
        );
        assert_eq!(launcher.launch(&::Method::GET, "/piyo/hoge"), None);
    }
}
