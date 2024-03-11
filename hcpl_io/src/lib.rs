#![doc = include_str!("../README.md")]

mod cin;
mod cout;
pub mod input_macro;
pub mod output_macro;
pub use cin::{Cin, Cinable};
pub use cout::{Cout, Coutable};

#[macro_export]
macro_rules! _io__prelude {
    ($cin:ident, $cout:ident) => {
        let stdin_handle = ::std::io::stdin();
        let stdout_handle = ::std::io::stdout();

        #[allow(unused_variables)]
        #[allow(unused_mut)]
        let mut $cin = hcpl_io::Cin::new(&stdin_handle);
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        let mut $cout = hcpl_io::Cout::new(&stdout_handle);

        hcpl_io::input_macro::make!($cin, $);
        hcpl_io::output_macro::make!($cout, $);
    };
}

pub use crate::_io__prelude as prelude;
