/// Set equipped with associative operation `op`
/// and identity `IDENTITY`.
///
/// Implementing this trait promises:
/// `Self::op(Self::op(a, b), c) = Self::op(a, Self::op(b, c))`
/// `Self::op(IDENTITY, a) = Self::op(a, IDENTITY) = a`
pub trait Monoid {
    const IDENTITY: Self;
    fn op(l: &Self, r: &Self) -> Self;
}

/// Monoid that acts on another monoid T.
///
/// Implementing this trait promises:
/// `Self::apply(f, T::op(a, b)) = T::op(Self::apply(f, a), Self::apply(f, b))`
pub trait MonoidAction<T: Monoid>: Monoid {
    fn apply(f: &Self, x: &T) -> T;
}

#[derive(Clone)]
pub struct AddMonoid<T>(pub T);
#[derive(Clone)]
pub struct MulMonoid<T>(pub T);
#[derive(Clone)]
pub struct MinMonoid<T>(pub T);
#[derive(Clone)]
pub struct MaxMonoid<T>(pub T);

impl<T: AdditiveIdentity> Monoid for AddMonoid<T>
where
    for<'r> &'r T: std::ops::Add<Output = T>,
{
    const IDENTITY: Self = AddMonoid(<T as AdditiveIdentity>::VALUE);

    fn op(AddMonoid(l): &Self, AddMonoid(r): &Self) -> Self {
        AddMonoid(l + r)
    }
}

impl<T: MultiplicativeIdentity> Monoid for MulMonoid<T>
where
    for<'r> &'r T: std::ops::Mul<Output = T>,
{
    const IDENTITY: Self = MulMonoid(<T as MultiplicativeIdentity>::VALUE);

    fn op(MulMonoid(l): &Self, MulMonoid(r): &Self) -> Self {
        MulMonoid(l * r)
    }
}

impl<T: MinimumIdentity> Monoid for MinMonoid<T>
where
    T: Clone,
    for<'r> &'r T: std::cmp::Ord,
{
    const IDENTITY: Self = MinMonoid(<T as MinimumIdentity>::VALUE);

    fn op(MinMonoid(l): &Self, MinMonoid(r): &Self) -> Self {
        MinMonoid(std::cmp::min(l, r).clone())
    }
}

pub trait AdditiveIdentity {
    const VALUE: Self;
}

pub trait MultiplicativeIdentity {
    const VALUE: Self;
}

pub trait MinimumIdentity {
    const VALUE: Self;
}

pub trait MaximumIdentity {
    const VALUE: Self;
}

macro_rules! impl_identities_for {
    ($t:ty) => {
        impl AdditiveIdentity for $t {
            const VALUE: Self = 0;
        }
        impl MultiplicativeIdentity for $t {
            const VALUE: Self = 1;
        }
        impl MinimumIdentity for $t {
            const VALUE: Self = Self::MAX;
        }
        impl MaximumIdentity for $t {
            const VALUE: Self = Self::MIN;
        }
    };
}

impl_identities_for!(u8);
impl_identities_for!(u16);
impl_identities_for!(u32);
impl_identities_for!(u64);
impl_identities_for!(u128);
impl_identities_for!(usize);
impl_identities_for!(i8);
impl_identities_for!(i16);
impl_identities_for!(i32);
impl_identities_for!(i64);
impl_identities_for!(i128);
impl_identities_for!(isize);
