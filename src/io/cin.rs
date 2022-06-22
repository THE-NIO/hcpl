use std::io::Read;

pub struct Cin<'a> {
    buffer: [u8; Cin::BUFFER_SIZE],
    begin: usize,
    end: usize,
    stdin: std::io::StdinLock<'a>,
    is_eof: bool,
}

impl<'a> Cin<'a> {
    const BUFFER_SIZE: usize = 1 << 17;

    pub fn new(stdin: &'a std::io::Stdin) -> Self {
        Self {
            buffer: [0; Self::BUFFER_SIZE],
            begin: 0,
            end: 0,
            stdin: stdin.lock(),
            is_eof: false,
        }
    }

    fn lshift_buffer(&mut self) {
        if self.begin != 0 {
            self.buffer.copy_within(self.begin..self.end, 0);
            self.end -= self.begin;
            self.begin = 0;
        }
    }

    pub fn refill(&mut self) {
        self.lshift_buffer();
        let read = self.stdin.read(&mut self.buffer[self.end..]).unwrap();
        if read == 0 {
            self.is_eof = true;
        } else {
            self.end += read;
        }
    }

    /// Read until predicate(byte) returns true.
    /// Returns a string containing the read characters,
    /// excluding the one where the predicate returned true.
    pub fn read_until<P: FnMut(u8) -> bool>(&mut self, mut predicate: P) -> Vec<u8> {
        let mut res = Vec::new();

        loop {
            match self.buffer[self.begin..self.end]
                .iter()
                .copied()
                .position(&mut predicate)
            {
                Some(pos) => {
                    res.extend_from_slice(&self.buffer[self.begin..self.begin + pos]);
                    self.begin += pos;
                    break;
                }
                None => {
                    res.extend_from_slice(&self.buffer[self.begin..self.end]);
                    self.begin = self.end;
                    self.refill();
                }
            }
        }

        res
    }

    /// Same as read_until, except the result is not stored.
    pub fn discard_until<P: FnMut(u8) -> bool>(&mut self, mut predicate: P) {
        loop {
            match self.buffer[self.begin..self.end]
                .iter()
                .copied()
                .position(&mut predicate)
            {
                Some(pos) => {
                    self.begin += pos;
                    break;
                }
                None => {
                    self.begin = self.end;
                    self.refill();
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
        cin.read_until(|_| if ate_one { true } else { ate_one = true; false })[0] as char
    }
}

macro_rules! read_integer_inner {
    ($t:ty, $cin:ident) => {{
        let mut res = 0;
        while !$cin.buffer[$cin.begin].is_ascii_whitespace() {
            res *= 10;
            res += ($cin.buffer[$cin.begin] - b'0') as $t;
            $cin.begin += 1;
        }
        res
    }};
}

macro_rules! make_unsigned_cinable {
    ($t:ty, $max_len:literal) => {
        impl Cinable for $t {
            fn read_from(cin: &mut Cin) -> Self {
                cin.discard_whitespace();

                if cin.end - cin.begin < $max_len {
                    cin.refill();
                }

                read_integer_inner!($t, cin)
            }
        }
    };
}

macro_rules! make_signed_cinable {
    ($t:ty, $unsigned_t:ty, $max_len:literal) => {
        impl Cinable for $t {
            fn read_from(cin: &mut Cin) -> Self {
                cin.discard_whitespace();

                if cin.end - cin.begin < $max_len {
                    cin.refill();
                }

                let neg = if cin.buffer[cin.begin] == b'-' {
                    cin.begin += 1;
                    true
                } else {
                    false
                };
                let res = read_integer_inner!($unsigned_t, cin);
                if neg {
                    res.overflowing_neg().0 as $t
                } else {
                    res as $t
                }
            }
        }
    };
}
make_unsigned_cinable!(u8, 3);
make_unsigned_cinable!(u16, 5);
make_unsigned_cinable!(u32, 10);
make_unsigned_cinable!(u64, 20);
make_unsigned_cinable!(u128, 39);
make_unsigned_cinable!(usize, 20); // TODO: calculate these numbers using const

make_signed_cinable!(i8, u8, 4);
make_signed_cinable!(i16, u16, 6);
make_signed_cinable!(i32, u32, 11);
make_signed_cinable!(i64, u64, 20);
make_signed_cinable!(i128, u128, 40);
make_signed_cinable!(isize, usize, 21); // TODO: calculate these numbers using const
