use std::io::Write;

pub struct Cout<'a> {
    buffer: [u8; Cout::BUFFER_SIZE],
    end: usize,
    stdout: std::io::StdoutLock<'a>,
}

impl<'a> Cout<'a> {
    const BUFFER_SIZE: usize = 1 << 17;

    pub fn new(stdout: &'a std::io::Stdout) -> Self {
        Self {
            buffer: [0; Self::BUFFER_SIZE],
            end: 0,
            stdout: stdout.lock(),
        }
    }

    pub fn flush(&mut self) {
        self.stdout.write_all(&self.buffer[..self.end]).unwrap();
        self.end = 0;
    }

    pub fn flush_if_too_long(&mut self, n: usize) {
        if self.end + n > Self::BUFFER_SIZE {
            self.flush()
        }
    }

    pub fn write<T: Coutable>(&mut self, x: T) -> &mut Self {
        x.write_to(self);
        self
    }
}

impl<'a> Drop for Cout<'a> {
    fn drop(&mut self) {
        self.flush();
    }
}

pub trait Coutable {
    fn write_to(&self, cout: &mut Cout);
}

impl Coutable for char {
    fn write_to(&self, cout: &mut Cout) {
        cout.flush_if_too_long(1);
        cout.buffer[cout.end] = *self as u8;
        cout.end += 1;
    }
}

impl Coutable for &str {
    fn write_to(&self, cout: &mut Cout) {
        for chunk in self.as_bytes().chunks(Cout::BUFFER_SIZE) {
            cout.flush_if_too_long(chunk.len());
            cout.buffer[cout.end..cout.end + chunk.len()].copy_from_slice(chunk);
            cout.end += chunk.len();
        }
    }
}

macro_rules! write_integer_inner {
    ($rest:ident, $buf:ident, $end:ident) => {
        loop {
            $end -= 1;
            $buf[$end] = ($rest % 10) as u8 + b'0';
            $rest /= 10;

            if $rest == 0 {
                break;
            }
        }
    };
}

macro_rules! make_unsigned_coutable {
    ($t:ty, $max_len:literal) => {
        impl Coutable for $t {
            fn write_to(&self, cout: &mut Cout) {
                cout.flush_if_too_long($max_len);

                let mut rest = *self;
                let mut buf = [0u8; $max_len];
                let mut end = $max_len;

                write_integer_inner!(rest, buf, end);

                cout.buffer[cout.end..cout.end + $max_len - end].copy_from_slice(&buf[end..]);
                cout.end += $max_len - end;
            }
        }
    };
}

macro_rules! make_signed_coutable {
    ($t:ty, $unsigned_t:ty, $max_len:literal) => {
        impl Coutable for $t {
            fn write_to(&self, cout: &mut Cout) {
                cout.flush_if_too_long($max_len);

                let neg = *self < 0;
                let mut rest = self.overflowing_abs().0 as $unsigned_t;
                let mut buf = [0u8; $max_len];
                let mut end = $max_len;

                write_integer_inner!(rest, buf, end);
                if neg {
                    end -= 1;
                    buf[end] = b'-';
                }

                cout.buffer[cout.end..cout.end + $max_len - end].copy_from_slice(&buf[end..]);
                cout.end += $max_len - end;
            }
        }
    };
}

make_unsigned_coutable!(u8, 3);
make_unsigned_coutable!(u16, 5);
make_unsigned_coutable!(u32, 10);
make_unsigned_coutable!(u64, 20);
make_unsigned_coutable!(u128, 39);
make_unsigned_coutable!(usize, 20); // TODO: calculate these numbers using const

make_signed_coutable!(i8, u8, 4);
make_signed_coutable!(i16, u16, 6);
make_signed_coutable!(i32, u32, 11);
make_signed_coutable!(i64, u64, 20);
make_signed_coutable!(i128, u128, 40);
make_signed_coutable!(isize, usize, 21); // TODO: calculate these numbers using const
