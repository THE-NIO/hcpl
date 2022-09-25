use super::monoid::{AdditiveIdentity, MultiplicativeIdentity};
use std::ops::{Add, Mul, Sub};

/// A ring.
pub trait Ring
where
    Self: Sized
        + AdditiveIdentity
        + MultiplicativeIdentity
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>,
{
}

impl<T> Ring for T where
    T: Sized
        + AdditiveIdentity
        + MultiplicativeIdentity
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
{
}
