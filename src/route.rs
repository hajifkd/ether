pub trait Route<A> {
    fn match_route(&self, method: ::Method, paths: &[&str]) -> Option<A>;
}

// TODO accept not only literals?
#[macro_export]
macro_rules! route {
    ( $method:expr ; $($tokens:tt),* ) => {{
        #[allow(non_camel_case_types)]
        struct __Ether_Route;

        impl $crate::route::Route<__route_extract_type!($($tokens),* ; )> for __Ether_Route {
            fn match_route(&self, method: $crate::Method, paths: &[&str])
                -> Option<__route_extract_type!($($tokens),* ; )> {
                if method != $method {
                    return None;
                }
                __route_impl!($($tokens),*; 0 ; ; ; paths);
            }
        }

        __Ether_Route{}
    }}
}

#[macro_export]
macro_rules! __route_convert_or {
    (($t:ty, $index:expr), $slice:expr) => {{
        if let Ok(r) = $slice[$index].parse::<$t>() {
            r
        } else {
            return None;
        }
    }};
}

#[macro_export]
macro_rules! __route_impl {
    ( $t:ty $(, $tokens:tt)* ; $num:expr ; $($match_index:expr),* ; $(($pt:ty, $parse_index:expr)),* ; $slice:expr) => {{
        __route_impl!($($tokens),* ; $num + 1 ; $($match_index),* ; $(($pt, $parse_index), )* ($t, $num) ; $slice)
    }};

    ( $path:expr $(, $tokens:tt)* ; $num:expr ; $($match_index:expr),* ; $(($pt:ty, $parse_index:expr)),* ; $slice:expr) => {{
        __route_impl!($($tokens),* ; $num + 1 ; $($match_index, )* ($path, $num) ; $(($pt, $parse_index)),* ; $slice)
    }};

    ( ; $num:expr ; $($match_index:expr),*; $(($t:ty, $parse_index:expr)),* ; $slice:expr) => {{
        if $num != $slice.len() {
            return None;
        }

        $({
            let (path, ind) = $match_index;

            if $slice[ind] != path {
                return None;
            }
        })*

        return Some(($({
            __route_convert_or!(($t, $parse_index), $slice)
        },)*));
    }};
}

#[macro_export]
macro_rules! __route_extract_type {
    ( $t:ty $(, $tokens:tt)* ; $($ts:ty)*) => {
        __route_extract_type!($($tokens),* ;$($ts)* $t)
    };

    ( $t:expr $(, $tokens:tt)* ; $($ts:ty)*) => {
        __route_extract_type!($($tokens),* ;$($ts)*)
    };

    ( ; $($ts:ty)* ) => (
        ($(
            $ts,
        )*)
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_extract_type() {
        fn hoge() -> __route_extract_type!(i32, "aaa", f64, "bbb", usize ; ) {
            (1i32, 1.0f64, 1usize)
        }

        assert_eq!(hoge(), (1i32, 1.0f64, 1usize));
    }

    #[test]
    fn test_convert_or() {
        assert_eq!(
            (|| Some(__route_convert_or!(
                (i32, 3),
                &["aa", "bb", "cc", "11", "dd"]
            )))(),
            Some(11)
        );

        assert_eq!(
            (|| Some(__route_convert_or!(
                (i32, 2),
                &["aa", "bb", "cc", "11", "dd"]
            )))(),
            None
        );
    }

    #[test]
    fn test_route() {
        use route::Route;

        // one argument
        let route = route!(::Method::Get; "test", "hoge", i32, "edit");
        assert_eq!(route.match_route(::Method::Get, &["aaa"]), None);
        assert_eq!(
            route.match_route(::Method::Get, &["test", "hoge", "fuga", "edit"]),
            None
        );
        assert_eq!(
            route.match_route(::Method::Get, &["test", "hoge", "42", "edit"]),
            Some((42,))
        );
        assert_eq!(
            route.match_route(::Method::Post, &["test", "hoge", "42", "edit"]),
            None
        );

        // multi arguments
        let route = route!(::Method::Get; "test", "hoge", i32, "edit", String, "hoge");
        assert_eq!(
            route.match_route(::Method::Get, &["test", "hoge", "42", "edit"]),
            None
        );
        assert_eq!(
            route.match_route(
                ::Method::Get,
                &["test", "hoge", "42", "edit", "bbb", "hoge"]
            ),
            Some((42, "bbb".to_owned()))
        );

        // no argument
        let route = route!(::Method::Get; "foo", "bar");
        assert_eq!(route.match_route(::Method::Get, &["aaa"]), None);
        assert_eq!(route.match_route(::Method::Get, &["foo", "bar"]), Some(()));
    }
}
