mod cin;
mod cout;
pub use cin::{Cin, Cinable};
pub use cout::{Cout, Coutable};

#[macro_export]
macro_rules! _io_prelude {
    ($cin:ident, $cout:ident) => {
        let stdin_handle = ::std::io::stdin();
        let stdout_handle = ::std::io::stdout();
        let mut $cin = hcpl::io::Cin::new(&stdin_handle);
        let mut $cout = hcpl::io::Cout::new(&stdout_handle);
    }
}

pub use crate::_io_prelude as prelude;
