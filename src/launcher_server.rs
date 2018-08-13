#[macro_export]
macro_rules! launcher {
    ([ $( $route:expr => $fn:expr );* ]) => {{
        use $crate::utils::apply;
        struct __Ether_Launcher;

        impl ether::Launcher for __Ether_Launcher {
            fn launch(&self, _m: &ether::Method, path: &str) -> Option<String> {
                $(
                    if let Some(a) = $route.match_route(path) {
                        return Some(apply($fn, a));
                    }
                );*

                return None;
            }
        }

        __Ether_Launcher
    }}
}
