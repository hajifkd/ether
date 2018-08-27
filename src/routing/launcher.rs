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
