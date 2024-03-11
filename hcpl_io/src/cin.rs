use std::io::BufRead;

use hcpl_integer::Integer;

#[cfg(feature = "derive")]
pub use hcpl_proc_macro::Cinable;

pub struct Cin<'a> {
    stdin: std::io::StdinLock<'a>,
}

impl<'a> Cin<'a> {
    pub fn new(stdin: &'a std::io::Stdin) -> Self {
        Self {
            stdin: stdin.lock(),
        }
    }

    pub fn buffer(&mut self) -> &[u8] {
        self.stdin.fill_buf().unwrap()
    }

    pub fn consume(&mut self, amt: usize) {
        self.stdin.consume(amt)
    }

    pub fn read_until<P: FnMut(u8) -> bool>(&mut self, mut predicate: P) -> Vec<u8> {
        let mut b = self.buffer();

        let mut res = Vec::new();

        loop {
            match b.iter().copied().position(&mut predicate) {
                Some(i) => {
                    res.extend_from_slice(&b[..i]);
                    self.consume(i);
                    break;
                }
                None => {
                    res.extend_from_slice(b);
                    let n = b.len();
                    self.consume(n);
                    b = self.buffer();
                }
            }
        }

        res
    }

    pub fn discard_until<P: FnMut(u8) -> bool>(&mut self, mut predicate: P) {
        let mut b = self.buffer();
        loop {
            match b.iter().copied().position(&mut predicate) {
                Some(i) => {
                    self.consume(i);
                    break;
                }
                None => {
                    let n = b.len();
                    self.consume(n);
                    b = self.buffer();
                }
            }
        }
    }

    pub fn discard_whitespace(&mut self) {
        self.discard_until(|b| !b.is_ascii_whitespace());
    }

    pub fn get<T: Cinable>(&mut self) -> T {
        T::read_from(self)
    }
}

pub trait Cinable {
    fn read_from(cin: &mut Cin) -> Self;
}

impl Cinable for char {
    fn read_from(cin: &mut Cin) -> Self {
        cin.discard_whitespace();
        let mut ate_one = false;
        cin.read_until(|_| {
            if ate_one {
                true
            } else {
                ate_one = true;
                false
            }
        })[0] as char
    }
}

impl Cinable for bool {
    fn read_from(cin: &mut Cin) -> Self {
        let c: char = cin.get();
        if c == '1' {
            true
        } else {
            assert_eq!(c, '0');
            false
        }
    }
}

macro_rules! read_integer_inner {
    ($t:ty, $cin:ident) => {{
        let mut res = 0;
        let mut b = $cin.buffer();
        debug_assert!(b.len() != 0);

        let mut p = 0;

        while {
            if p == b.len() {
                $cin.consume(p);
                p = 0;
                b = $cin.buffer();
                debug_assert!(b.len() != 0);
            }
            !b[p].is_ascii_whitespace()
        } {
            res *= 10;
            res += (b[p] - b'0') as $t;
            p += 1;
        }
        $cin.consume(p);
        res
    }};
}

macro_rules! make_unsigned_cinable {
    ($t:ty) => {
        impl Cinable for $t {
            fn read_from(cin: &mut Cin) -> Self {
                cin.discard_whitespace();
                read_integer_inner!($t, cin)
            }
        }
    };
}

macro_rules! make_signed_cinable {
    ($t:ty) => {
        impl Cinable for $t {
            fn read_from(cin: &mut Cin) -> Self {
                cin.discard_whitespace();

                let b = cin.buffer();
                debug_assert!(b.len() != 0);

                let neg = if b[0] == b'-' {
                    cin.consume(1);
                    true
                } else {
                    false
                };

                let res = read_integer_inner!(<$t as Integer>::AsUnsigned, cin);
                if neg {
                    res.overflowing_neg().0 as $t
                } else {
                    res as $t
                }
            }
        }
    };
}
make_unsigned_cinable!(u8);
make_unsigned_cinable!(u16);
make_unsigned_cinable!(u32);
make_unsigned_cinable!(u64);
make_unsigned_cinable!(u128);
make_unsigned_cinable!(usize);

make_signed_cinable!(i8);
make_signed_cinable!(i16);
make_signed_cinable!(i32);
make_signed_cinable!(i64);
make_signed_cinable!(i128);
make_signed_cinable!(isize);
