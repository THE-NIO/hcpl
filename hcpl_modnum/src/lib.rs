use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// An element of ℤ/mℤ where m = MOD. Implemented over `u32`.
pub struct Modnum<const MOD: u32>(u32);

impl<const MOD: u32> Display for Modnum<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <u32 as Display>::fmt(&self.0, f)
    }
}

impl<const MOD: u32> Modnum<MOD> {
    /// Constructs a `Modnum<MOD>` from a `u32`. Equivalent to `Modnum::from(x)`
    pub const fn new(x: u32) -> Self {
        // Self::from(x)  // Requires const trait impls
        Self(x % MOD)
    }

    /// Modular exponentiation by squaring.
    pub fn pow(mut self, mut e: usize) -> Self {
        let mut a = Self::new(1);
        while e != 0 {
            if e & 1 == 1 {
                a *= self;
            }
            self *= self;

            e >>= 1;
        }
        a
    }

    /// Calculates the inverse of `self`, **assuming `MOD` is prime**.
    pub fn inv(self) -> Self {
        self.pow(MOD as usize - 2)
    }
}

impl<const MOD: u32> From<u32> for Modnum<MOD> {
    fn from(n: u32) -> Self {
        Self(n % MOD)
    }
}

impl<const MOD: u32> From<usize> for Modnum<MOD> {
    fn from(x: usize) -> Self {
        Self((x % MOD as usize) as u32)
    }
}

impl<const MOD: u32> From<Modnum<MOD>> for u32 {
    fn from(x: Modnum<MOD>) -> u32 {
        x.0
    }
}

impl<const MOD: u32> Add for Modnum<MOD> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let res = self.0 + rhs.0;
        Self(if res >= MOD { res - MOD } else { res })
    }
}

impl<const MOD: u32> Neg for Modnum<MOD> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self.0 == 0 {
            self
        } else {
            Self(MOD - self.0)
        }
    }
}

impl<const MOD: u32> Sub for Modnum<MOD> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<const MOD: u32> Mul for Modnum<MOD> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self((self.0 as u64 * rhs.0 as u64 % MOD as u64) as u32)
    }
}

impl<const MOD: u32> Div for Modnum<MOD> {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl<const MOD: u32> AddAssign for Modnum<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const MOD: u32> SubAssign for Modnum<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const MOD: u32> MulAssign for Modnum<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const MOD: u32> DivAssign for Modnum<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const MOD: u32> hcpl_algebra::monoid::AdditiveIdentity for Modnum<MOD> {
    const VALUE: Self = Self::new(0);
}

impl<const MOD: u32> hcpl_algebra::monoid::MultiplicativeIdentity for Modnum<MOD> {
    const VALUE: Self = Self::new(1);
}
