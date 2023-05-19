#[derive(Clone, Copy, Debug, Default)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub const ZERO: Complex = Complex { re: 0., im: 0. };
    pub const ONE: Complex = Complex { re: 1., im: 0. };
    pub const I: Complex = Complex { re: 0., im: 1. };

    pub fn powi(mut self, mut rhs: u32) -> Self {
        let mut result = Self::ONE;

        while rhs > 0 {
            if (rhs & 1) != 0 {
                result *= self;
            }

            self *= self;
            rhs >>= 1;
        }

        result
    }

    pub fn cis(angle: f64) -> Self {
        Self {
            re: angle.cos(),
            im: angle.sin(),
        }
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} + i {}", self.re, self.im))
    }
}

impl From<f64> for Complex {
    fn from(value: f64) -> Self {
        Self { re: value, im: 0. }
    }
}

impl std::ops::Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl std::ops::Add<f64> for Complex {
    type Output = Complex;

    fn add(self, rhs: f64) -> Self::Output {
        self + Complex::from(rhs)
    }
}

impl std::ops::Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Self::Output {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl std::ops::Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Self::Output {
        self + -rhs
    }
}

impl std::ops::Sub<f64> for Complex {
    type Output = Complex;

    fn sub(self, rhs: f64) -> Self::Output {
        self + -rhs
    }
}

impl std::ops::Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl std::ops::Mul<f64> for Complex {
    type Output = Complex;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

impl std::ops::Div<f64> for Complex {
    type Output = Complex;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

impl std::ops::AddAssign<Complex> for Complex {
    fn add_assign(&mut self, rhs: Complex) {
        *self = *self + rhs;
    }
}

impl std::ops::AddAssign<f64> for Complex {
    fn add_assign(&mut self, rhs: f64) {
        *self = *self + rhs;
    }
}

impl std::ops::SubAssign<Complex> for Complex {
    fn sub_assign(&mut self, rhs: Complex) {
        *self = *self - rhs;
    }
}

impl std::ops::SubAssign<f64> for Complex {
    fn sub_assign(&mut self, rhs: f64) {
        *self = *self - rhs;
    }
}

impl std::ops::MulAssign<Complex> for Complex {
    fn mul_assign(&mut self, rhs: Complex) {
        *self = *self * rhs;
    }
}

impl std::ops::MulAssign<f64> for Complex {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl std::ops::DivAssign<f64> for Complex {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl std::iter::Sum<Complex> for Complex {
    fn sum<I: Iterator<Item = Complex>>(iter: I) -> Self {
        let mut result = Complex::ZERO;

        for item in iter {
            result += item;
        }

        result
    }
}

impl std::iter::Product<Complex> for Complex {
    fn product<I: Iterator<Item = Complex>>(iter: I) -> Self {
        let mut result = Complex::ONE;

        for item in iter {
            result *= item;
        }

        result
    }
}