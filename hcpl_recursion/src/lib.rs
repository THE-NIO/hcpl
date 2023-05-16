#[macro_export]
macro_rules! _recursion__let_rec {
    ($f:ident = |$($arg_id:ident: $arg_ty:ty),*| -> $ret:ty $body:block) => {
        let $f = {
            trait AlmostFTrait {
                fn call(&self, $($arg_id: $arg_ty),*) -> $ret;
            }
            struct Almost<F>(F);
            impl<F: Fn(&dyn AlmostFTrait $(,$arg_ty)*) -> $ret> AlmostFTrait for Almost<F> {
                #[inline(always)]
                fn call(&self, $($arg_id: $arg_ty),*) -> $ret {
                    (self.0)(self, $($arg_id),*)
                }
            }
            let almost_f = Almost(|almost_f: &dyn AlmostFTrait,  $($arg_id: $arg_ty),*| -> $ret {
                let $f = |$($arg_id: $arg_ty),*| {
                    almost_f.call($($arg_id),*)
                };
                $body
            });
            move |$($arg_id: $arg_ty),*| {
                almost_f.call($($arg_id),*)
            }
        };
    }
}

#[macro_export]
macro_rules! _recursion__let_rec_mut__impl {
    ($f:ident = [ $($cap_id:ident: $cap_ty:ty),* $(,)? ] |$($arg_id:ident: $arg_ty:ty),*| -> $ret:ty $body:block $dol:tt) => {
        let $f = {
            hcpl_recursion::let_rec!(without_cap = |$($cap_id: &mut $cap_ty,)* $($arg_id: $arg_ty),*| -> $ret {
                #[allow(unused_macros)]
                macro_rules! $f {
                    ($dol ($dol inner_args:tt)*) => {
                        without_cap($($cap_id,)* $dol ($dol inner_args)*)
                    }
                }
                $body
            });
            without_cap
        };
        macro_rules! $f {
            ($dol ($dol inner_args:tt),*) => {
                $f($(&mut $cap_id,)* $dol ($dol inner_args,)*)
            }
        }
    };
    ($f:ident = [ $($cap_id:ident: $cap_ty:ty),* $(,)? ] || -> $ret:ty $body:block $dol:tt) => {
        let $f = {
            hcpl_recursion::let_rec!(without_cap = |$($cap_id: &mut $cap_ty),*| -> $ret {
                #[allow(unused_macros)]
                macro_rules! $f {
                    ($dol ($dol inner_args:tt)*) => {
                        without_cap($($cap_id,)* $dol ($dol inner_args)*)
                    }
                }
                $body
            });
            without_cap
        };
        macro_rules! $f {
            () => {
                $f($(&mut $cap_id),*)
            }
        }
    };
}

#[macro_export]
macro_rules! _recursion__let_rec_mut {
    ($($args:tt)*) => {
        hcpl_recursion::_recursion__let_rec_mut__impl!($($args)* $);
    }
}

pub use crate::{
    _recursion__let_rec as let_rec,
    _recursion__let_rec_mut as let_rec_mut,
};
