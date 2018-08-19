// See https://bit.ly/2nzMCVm
#[inline(always)]
pub fn apply<Fun, Pre, In, Out>(f: Fun, pre: Pre, p: In) -> Out
where
    ApplyImpl: Apply<Fun, Pre, In, Out>,
{
    ApplyImpl::apply(f, pre, p)
}

pub struct ApplyImpl;

pub trait Apply<Fun, Pre, In, Out> {
    fn apply(f: Fun, pre_param: Pre, params: In) -> Out;
}

macro_rules! gen_apply {
    () => {
        impl<Fun, Pre, Out> Apply<Fun, Pre, (), Out> for ApplyImpl
        where
            Fun: Fn(Pre) -> Out,
        {
            #[inline(always)]
            fn apply(f: Fun, pre_param: Pre, _p: ()) -> Out {
                f(pre_param)
            }
        }
    };

    ( $S:ident $(, $T:ident )* ) => {
        gen_apply!{ $($T),* }

        impl<Fun, Pre, $S, $($T, )* Out> Apply<Fun, Pre, ($S, $($T),*), Out> for ApplyImpl
        where
            Fun: Fn(Pre, $S $(, $T)*) -> Out,
        {
            #[inline(always)]
            fn apply(f: Fun, pre_param: Pre, p: ($S, $($T),*)) -> Out {
                #[allow(non_snake_case)]
                let ($S, $($T),*) = p;
                f(pre_param, $S $(, $T)*)
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
        assert_eq!(apply(i32::add, 1, (2,)), 3);
    }
}
