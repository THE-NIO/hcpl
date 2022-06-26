#[macro_export]
macro_rules! _io__output_macro__make {
    ($cout:ident, $dol:tt) => {
        #[allow(unused_macros)]
        macro_rules! out {
            ($dol ($dol tail:tt)*) => {
                hcpl_io::output_macro::out_to!($cout; $dol ($dol tail)*)
            };
        }
        #[allow(unused_macros)]
        macro_rules! outln {
            ($dol ($dol tail:tt)*) => {
                hcpl_io::output_macro::out_to!($cout; $dol ($dol tail)*; '\n')
            };
        }
    };
}

#[macro_export]
macro_rules! _io__output_macro__out_to {
    ($cout:ident; $head:expr, $($tail:tt)*) => {
        $cout.write($head).write(' ');
        hcpl_io::output_macro::out_to!($cout; $($tail)*);
    };
    ($cout:ident; $head:expr; $($tail:tt)*) => {
        $cout.write($head);
        hcpl_io::output_macro::out_to!($cout; $($tail)*);
    };
    ($cout:ident; $head:expr) => {
        $cout.write($head);
    };
    ($cout:ident;) => {};
}

pub use crate::{
    _io__output_macro__make as make,
    _io__output_macro__out_to as out_to,
};
