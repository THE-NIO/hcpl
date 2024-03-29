use std::cmp::Ord;
use std::ops::*;

/// A trait for integer types.
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
    /// The unsigned variant of this type.
    type AsUnsigned;
    /// The signed variant of this type.
    type AsSigned;
    /// Zero.
    const ZERO: Self;
    /// One.
    const ONE: Self;
    /// Ten.
    const TEN: Self;
    /// The length of the largest number of this type, in base ten.
    const BASE_10_MAX_LENGTH: usize;
    /// Returns the number of leading ones in the binary representation of `self`.
    fn leading_ones(self) -> u32;
    /// Returns the number of leading zeros in the binary representation of `self`.
    fn leading_zeros(self) -> u32;
    /// Returns the number of trailing ones in the binary representation of `self`.
    fn trailing_ones(self) -> u32;
    /// Returns the number of trailing zeros in the binary representation of `self`.
    fn trailing_zeros(self) -> u32;
    /// Returns the absolute value of `self` as an unsigned integer without panicking.
    fn unsigned_abs(self) -> Self::AsUnsigned;
    /// Returns `true` if `self == Self::ZERO`.
    fn is_zero(self) -> bool {
        self == Self::ZERO
    }
}

/// An unsigned integer type.
pub trait UnsignedInteger: Integer {}
/// A signed integer type.
pub trait SignedInteger: Integer {}

macro_rules! make_int {
    ($t:ty, $ut:ty, $it:ty, $unsigned_abs:expr, $base_10_max_len:expr) => {
        impl Integer for $t {
            type AsUnsigned = $ut;
            type AsSigned = $it;
            const ZERO: $t = 0;
            const ONE: $t = 1;
            const TEN: $t = 10;
            const BASE_10_MAX_LENGTH: usize = $base_10_max_len;
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

macro_rules! base_10_len {
    ($v:expr) => {{
        let mut x = $v;
        let mut ans = 0;
        loop {
            ans += 1;
            x /= 10;

            if x == 0 {
                break;
            }
        }
        ans
    }};
}

macro_rules! make_ints {
    ($ut:ty, $it:ty) => {
        make_int!($ut, $ut, $it, |x| x, base_10_len!(<$ut>::MAX));
        make_int!(
            $it,
            $ut,
            $it,
            Self::unsigned_abs,
            1 + base_10_len!(<$it>::MIN.unsigned_abs())
        );
        impl UnsignedInteger for $ut {}
        impl SignedInteger for $it {}
    };
}

make_ints!(u8, i8);
make_ints!(u16, i16);
make_ints!(u32, i32);
make_ints!(u64, i64);
make_ints!(u128, i128);
make_ints!(usize, isize);
