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

    pub fn conj(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    pub fn squared_norm(self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    pub fn inv(self) -> Self {
        Self {
            re: self.re / self.squared_norm(),
            im: -self.im / self.squared_norm(),
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

impl From<usize> for Complex {
    fn from(value: usize) -> Self {
        Self {
            re: value as f64,
            im: 0.,
        }
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

impl std::ops::Div<Complex> for Complex {
    type Output = Complex;

    fn div(self, rhs: Complex) -> Self::Output {
        self * rhs.inv()
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

impl std::ops::DivAssign<Complex> for Complex {
    fn div_assign(&mut self, rhs: Complex) {
        *self = *self / rhs;
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

impl hcpl_algebra::monoid::AdditiveIdentity for Complex {
    const VALUE: Self = Complex::ZERO;
}
impl hcpl_algebra::monoid::MultiplicativeIdentity for Complex {
    const VALUE: Self = Complex::ONE;
}

impl hcpl_number_theory::roots::TryNthRootOfUnity for Complex {
    type Error = std::convert::Infallible;

    fn try_nth_root_of_unity(n: usize) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(Self::cis(2. * std::f64::consts::PI / n as f64))
    }

    fn try_nth_root_of_unity_inv(n: usize) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(Self::cis(-2. * std::f64::consts::PI / n as f64))
    }
}
