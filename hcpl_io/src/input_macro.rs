#[macro_export]
macro_rules! _io__input_macro__make {
    ($cin:ident, $dol:tt) => {
        #[allow(unused_macros)]
        macro_rules! input {
            ($dol ($dol tail:tt)*) => {
                hcpl_io::input_macro::input_from!($cin; $dol ($dol tail)*)
            };
        }
    };
}

#[macro_export]
macro_rules! _io__input_macro__input_from {
    ($cin:ident; $var:ident : $type:tt $(+ $offset:literal)? $(- $noffset:literal)?, $($tail:tt)*) => {
        let $var = hcpl_io::input_macro::read_value!($cin; $type $(+ $offset)? $(- $noffset)?);
        hcpl_io::input_macro::input_from!($cin; $($tail)*);
    };
    ($cin:ident; mut $var:ident : $type:tt $(+ $offset:literal)? $(- $noffset:literal)?, $($tail:tt)*) => {
        let mut $var = hcpl_io::input_macro::read_value!($cin; $type $(+ $offset)? $(- $noffset)?);
        hcpl_io::input_macro::input_from!($cin; $($tail)*);
    };
    ($cin:ident;) => {};
}

#[macro_export]
macro_rules! _io__input_macro__read_value {
    ($cin:ident; ( $($inner:tt $(+ $offset:literal)? $(- $noffset:literal)?),* )) => {
        ( $(hcpl_io::input_macro::read_value!($cin; $inner $(+ $offset)? $(- $noffset)?)),* )
    };
    ($cin:ident; [ $inner:tt $(+ $offset:literal)? $(- $noffset:literal)?; $n:expr ]) => {
        (0..$n).map(|_| hcpl_io::input_macro::read_value!($cin; $inner $(+ $offset)? $(- $noffset)?)).collect::<Vec<_>>()
    };
    ($cin:ident; bytes) => {{
        $cin.discard_whitespace();
        $cin.read_until(|b| b.is_ascii_whitespace())
    }};
    ($cin:ident; $type:tt $(+ $offset:literal)? $(- $noffset:literal)?) => {
        ($cin.get::<$type>() $(- $offset)? $(+ $noffset)?)
    };
}

pub use crate::{
    _io__input_macro__input_from as input_from, _io__input_macro__make as make,
    _io__input_macro__read_value as read_value,
};
