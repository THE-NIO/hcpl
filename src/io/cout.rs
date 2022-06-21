use std::io::Write;

pub struct Cout<'a> {
    buffer: [u8; Cout::BUFFER_SIZE],
    end: usize,
    stdout: std::io::StdoutLock<'a>,
}

impl<'a> Cout<'a> {
    const BUFFER_SIZE: usize = 1 << 17;
    const MAX_INT_LEN: usize = 32;

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

impl Coutable for usize {
    fn write_to(&self, cout: &mut Cout) {
        cout.flush_if_too_long(Cout::MAX_INT_LEN);

        let mut rest = *self;
        let mut buf = [0u8; Cout::MAX_INT_LEN];
        let mut end = Cout::MAX_INT_LEN;

        loop {
            end -= 1;
            buf[end] = (rest % 10) as u8 + b'0';
            rest /= 10;

            if rest == 0 {
                break;
            }
        }

        while end < Cout::MAX_INT_LEN {
            cout.buffer[cout.end] = buf[end];
            cout.end += 1;
            end += 1;
        }
    }
}
