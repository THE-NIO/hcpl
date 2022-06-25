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
        let mut $cin = hcpl::io::Cin::new(&stdin_handle);
        #[allow(unused_variables)]
        let mut $cout = hcpl::io::Cout::new(&stdout_handle);

        hcpl::io::input_macro::make!($cin, $);
        hcpl::io::output_macro::make!($cout, $);
    };
}

pub use crate::_io__prelude as prelude;
