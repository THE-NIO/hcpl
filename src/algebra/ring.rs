use std::ops::{Add, Mul, Sub};

pub trait Ring:
    Sized + Add<Self, Output = Self> + Sub<Self, Output = Self> + Mul<Self, Output = Self>
{
    const ADDITIVE_IDENTITY: Self;
    const MULTIPLICATIVE_IDENTITY: Self;
}

macro_rules! impl_ring_for {
    ($t:ty) => {
        impl Ring for $t {
            const ADDITIVE_IDENTITY: Self = 0;
            const MULTIPLICATIVE_IDENTITY: Self = 1;
        }
    };
}

impl_ring_for!(u8);
impl_ring_for!(u16);
impl_ring_for!(u32);
impl_ring_for!(u64);
impl_ring_for!(u128);
impl_ring_for!(usize);
impl_ring_for!(i8);
impl_ring_for!(i16);
impl_ring_for!(i32);
impl_ring_for!(i64);
impl_ring_for!(i128);
impl_ring_for!(isize);
