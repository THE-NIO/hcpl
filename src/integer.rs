use std::cmp::Ord;
use std::ops::*;

pub trait Integer:
    Copy
    + Clone
    + Eq
    + Ord
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + Shl<Self, Output = Self>
    + Shr<Self, Output = Self>
    + Shl<u32, Output = Self>
    + Shr<u32, Output = Self>
    + ShlAssign<Self>
    + ShrAssign<Self>
    + ShlAssign<u32>
    + ShrAssign<u32>
{
    type AsUnsigned;
    type AsSigned;
    const WIDTH: usize;
    const ZERO: Self;
    fn leading_ones(self) -> u32;
    fn leading_zeros(self) -> u32;
    fn trailing_ones(self) -> u32;
    fn trailing_zeros(self) -> u32;
    fn unsigned_abs(self) -> Self::AsUnsigned;
    fn is_zero(self) -> bool {
        self == Self::ZERO
    }
}

pub trait UnsignedInteger: Integer {}
pub trait SignedInteger: Integer {}

macro_rules! make_int {
    ($t:ty, $ut:ty, $it:ty, $width:literal, $unsigned_abs:expr) => {
        impl Integer for $t {
            type AsUnsigned = $ut;
            type AsSigned = $it;
            const WIDTH: usize = $width;
            const ZERO: $t = 0;
            fn leading_ones(self) -> u32 {
                <$t>::leading_ones(self)
            }
            fn leading_zeros(self) -> u32 {
                <$t>::leading_zeros(self)
            }
            fn trailing_ones(self) -> u32 {
                <$t>::trailing_ones(self)
            }
            fn trailing_zeros(self) -> u32 {
                <$t>::trailing_zeros(self)
            }
            fn unsigned_abs(self) -> Self::AsUnsigned {
                $unsigned_abs(self)
            }
        }
    };
}

macro_rules! make_ints {
    ($ut:ty, $it:ty, $width:literal) => {
        make_int!($ut, $ut, $it, $width, |x| x);
        make_int!($it, $ut, $it, $width, Self::unsigned_abs);
        impl UnsignedInteger for $ut {}
        impl SignedInteger for $it {}
    };
}

make_ints!(u8, i8, 8);
make_ints!(u16, i16, 16);
make_ints!(u32, i32, 32);
make_ints!(u64, i64, 64);
make_ints!(u128, i128, 128);
