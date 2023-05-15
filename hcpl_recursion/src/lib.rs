#[macro_export]
macro_rules! _recursion__let_rec {
    ($f:ident = |$($arg_id:ident: $arg_ty:ty),*| -> $ret:ty $body:block) => {
        trait AlmostFTrait {
            fn call(&self, $($arg_id: $arg_ty),*) -> $ret;
        }
        struct Almost<F>(F);
        let almost_f = Almost(|almost_f: &dyn AlmostFTrait,  $($arg_id: $arg_ty),*| -> $ret {
            let $f = |$($arg_id: $arg_ty),*| {
                almost_f.call($($arg_id),*)
            };
            $body
        });
        impl<F: Fn(&dyn AlmostFTrait $(,$arg_ty)*) -> $ret> AlmostFTrait for Almost<F> {
            fn call(&self, $($arg_id: $arg_ty),*) -> $ret {
                (self.0)(self, $($arg_id),*)
            }
        }
        let $f = |$($arg_id: $arg_ty),*| {
            almost_f.call($($arg_id),*)
        };
    }
}

#[macro_export]
macro_rules! _recursion__let_rec_mut__impl {
    ([$($cap_id:ident: $cap_ty:ty,)*] [$($mut_cap_id:ident: $mut_cap_ty:ty,)*] [$head:ident: $head_type:ty, $($tail:tt)*] $($rest:tt)*) => {
        hcpl_recursion::_recursion__let_rec_mut__impl!([$($cap_id: $cap_ty,)* $head: $head_type,] [$($mut_cap_id: $mut_cap_ty,)*] [$($tail)*] $($rest)*);
    };
    ([$($cap_id:ident: $cap_ty:ty,)*] [$($mut_cap_id:ident: $mut_cap_ty:ty,)*] [mut $head:ident: $head_type:ty, $($tail:tt)*] $($rest:tt)*) => {
        hcpl_recursion::_recursion__let_rec_mut__impl!([$($cap_id: $cap_ty,)*] [$($mut_cap_id: $mut_cap_ty,)* $head: $head_type,] [$($tail)*] $($rest)*);
    };
    ([$($cap_id:ident: $cap_ty:ty,)*] [$($mut_cap_id:ident: $mut_cap_ty:ty,)*] [$head:ident: $head_type:ty] $($rest:tt)*) => {
        hcpl_recursion::_recursion__let_rec_mut__impl!([$($cap_id: $cap_ty,)* $head: $head_type,] [$($mut_cap_id: $mut_cap_ty,)*] [] $($rest)*);
    };
    ([$($cap_id:ident: $cap_ty:ty,)*] [$($mut_cap_id:ident: $mut_cap_ty:ty,)*] [mut $head:ident: $head_type:ty] $($rest:tt)*) => {
        hcpl_recursion::_recursion__let_rec_mut__impl!([$($cap_id: $cap_ty,)*] [$($mut_cap_id: $mut_cap_ty,)* $head: $head_type,] [] $($rest)*);
    };
    ([$($cap_id:ident: $cap_ty:ty,)*] [$($mut_cap_id:ident: $mut_cap_ty:ty,)*] [] $f:ident = |$($arg_id:ident: $arg_ty:ty),*| -> $ret:ty $body:block $dol:tt) => {
        fn call($($cap_id: &$cap_ty,)* $($mut_cap_id: &mut $mut_cap_ty,)* $($arg_id: $arg_ty),*) -> $ret {
            macro_rules! $f {
                ($dol ($dol inner_args:tt),*) => {
                    call($($cap_id,)* $($mut_cap_id,)* $dol ($dol inner_args),*)
                };
            }
            $body
        }
        let mut $f = |$($arg_id: $arg_ty),*| {
            call($(&$cap_id,)* $(&mut $mut_cap_id,)* $($arg_id),*);
        };
    }
}

#[macro_export]
macro_rules! _recursion__let_rec_mut {
    ($f:ident = [$($captures:tt)*] |$($arg_id:ident: $arg_ty:ty),*| -> $ret:ty $body:block) => {
        hcpl_recursion::_recursion__let_rec_mut__impl!([] [] [$($captures)*] $f = |$($arg_id: $arg_ty),*| -> $ret $body $);
    };
}

pub use crate::{
    _recursion__let_rec as let_rec,
    _recursion__let_rec_mut as let_rec_mut,
};
