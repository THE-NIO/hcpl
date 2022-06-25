#[macro_export]
macro_rules! _io__output_macro__make {
    ($cout:ident, $dol:tt) => {
        macro_rules! out {
            ($dol ($dol tail:tt)*) => {
                hcpl::io::output_macro::out_to!($cout; $dol ($dol tail)*)
            };
        }
        macro_rules! outln {
            ($dol ($dol tail:tt)*) => {
                hcpl::io::output_macro::out_to!($cout; $dol ($dol tail)* '\n')
            };
        }
    };
}

#[macro_export]
macro_rules! _io__output_macro__out_to {
    ($cout:ident; $head:expr, $($tail:tt)*) => {
        $cout.write($head).write(' ');
        hcpl::io::output_macro::out_to!($cout; $($tail)*);
    };
    ($cout:ident; $head:tt $($tail:tt)*) => {
        $cout.write($head);
        hcpl::io::output_macro::out_to!($cout; $($tail)*);
    };
    ($cout:ident;) => {};
}

pub use crate::{
    _io__output_macro__make as make,
    _io__output_macro__out_to as out_to,
};
