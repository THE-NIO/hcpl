/// Set equipped with associative operation `op`
/// and identity `IDENTITY`.
///
/// Implementations of this trait promise:
/// `Self::op(Self::op(a, b), c) = Self::op(a, Self::op(b, c))`
/// `Self::op(IDENTITY, a) = Self::op(a, IDENTITY) = a`
pub trait Monoid {
    /// The identity element of the monoid.
    const IDENTITY: Self;

    /// The monoid operation.
    fn op(l: Self, r: Self) -> Self;
}

/// Monoid that acts on another monoid T.
///
/// Implementations of this trait must respect the following laws:
/// - `apply` must define a monoid action, ie.
///   `Self::apply(f, Self::apply(g, a)) = Self::apply(Self::op(f, g), a)`
///   `Self::apply(Self::IDENTITY, a) = a`
/// - the monoid action must preserve the structure of the monoid it acts on, ie.
///   `Self::apply(f, T::op(a, b)) = T::op(Self::apply(f, a), Self::apply(f, b))`
pub trait MonoidAction<T: Monoid>: Monoid {
    fn apply(f: &Self, x: &T) -> T;
}

impl Monoid for () {
    const IDENTITY: Self = ();
    fn op(_: Self, _: Self) -> Self {}
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct AddMonoid<T>(pub T);
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct MulMonoid<T>(pub T);
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct MinMonoid<T>(pub T);
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct MaxMonoid<T>(pub T);

impl<T: AdditiveIdentity> Monoid for AddMonoid<T>
where
    T: std::ops::Add<Output = T>,
{
    const IDENTITY: Self = AddMonoid(<T as AdditiveIdentity>::VALUE);

    fn op(AddMonoid(l): Self, AddMonoid(r): Self) -> Self {
        AddMonoid(l + r)
    }
}

impl<T: MultiplicativeIdentity> Monoid for MulMonoid<T>
where
    T: std::ops::Mul<Output = T>,
{
    const IDENTITY: Self = MulMonoid(<T as MultiplicativeIdentity>::VALUE);

    fn op(MulMonoid(l): Self, MulMonoid(r): Self) -> Self {
        MulMonoid(l * r)
    }
}

impl<T: MinimumIdentity> Monoid for MinMonoid<T>
where
    T: Clone,
    T: std::cmp::Ord,
{
    const IDENTITY: Self = MinMonoid(<T as MinimumIdentity>::VALUE);

    fn op(MinMonoid(l): Self, MinMonoid(r): Self) -> Self {
        MinMonoid(std::cmp::min(l, r))
    }
}

impl<T: MaximumIdentity> Monoid for MaxMonoid<T>
where
    T: Clone,
    T: std::cmp::Ord,
{
    const IDENTITY: Self = MaxMonoid(<T as MaximumIdentity>::VALUE);

    fn op(MaxMonoid(l): Self, MaxMonoid(r): Self) -> Self {
        MaxMonoid(std::cmp::max(l, r))
    }
}

/// Trait for types with an additive identity.
///
/// The law `<Self as AdditiveIdentity>::VALUE + n = n` should be satisfied.
pub trait AdditiveIdentity {
    /// The additive identity of `Self`.
    const VALUE: Self;
}

/// Trait for types with an multiplicative identity.
///
/// The law `<Self as MultiplicativeIdentity>::VALUE * n = n` should be satisfied.
pub trait MultiplicativeIdentity {
    /// The multiplicative identity of `Self`.
    const VALUE: Self;
}

/// Trait for types with an identity with respect to [`std::cmp::min`].
///
/// The law `<Self as MinimumIdentity>::VALUE.min(n) = n` should be satisfied.
pub trait MinimumIdentity {
    /// The minimum identity of `Self`.
    const VALUE: Self;
}

/// Trait for types with an identity with respect to [`std::cmp::max`].
///
/// The law `<Self as MaximumIdentity>::VALUE.max(n) = n` should be satisfied.
pub trait MaximumIdentity {
    /// The maximum identity of `Self`.
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
