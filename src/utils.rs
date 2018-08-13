// See https://bit.ly/2nzMCVm
#[inline(always)]
pub fn apply<Fun, In, Out>(f: Fun, p: In) -> Out
where
    ApplyImpl: Apply<Fun, In, Out>,
{
    ApplyImpl::apply(f, p)
}

pub struct ApplyImpl;

pub trait Apply<Fun, In, Out> {
    fn apply(f: Fun, params: In) -> Out;
}

macro_rules! gen_apply {
    () => {
        impl<Fun, Out> Apply<Fun, (), Out> for ApplyImpl
        where
            Fun: Fn() -> Out,
        {
            #[inline(always)]
            fn apply(f: Fun, _p: ()) -> Out {
                f()
            }
        }
    };

    ( $S:ident $(, $T:ident )* ) => {
        gen_apply!{ $($T),* }

        impl<Fun, $S, $($T, )* Out> Apply<Fun, ($S, $($T),*), Out> for ApplyImpl
        where
            Fun: Fn($S $(, $T)*) -> Out,
        {
            #[inline(always)]
            fn apply(f: Fun, p: ($S, $($T),*)) -> Out {
                #[allow(non_snake_case)]
                let ($S, $($T),*) = p;
                f($S $(, $T)*)
            }
        }
    };
}

gen_apply!{A, B, C, D, E, F, G, H, I}

#[cfg(test)]
mod tests {
    use utils::apply;

    #[test]
    fn test_apply() {
        use std::ops::Add;
        assert_eq!(apply(i32::add, (1, 2)), 3);
    }
}
