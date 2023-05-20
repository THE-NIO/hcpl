use hcpl_algebra::{monoid::MultiplicativeIdentity, Ring};

pub trait TryNthRootOfUnity: Sized + Copy + Ring {
    type Error;

    /// Returns a principal nth root of unity. Must be deterministic.
    fn try_nth_root_of_unity(n: usize) -> Result<Self, Self::Error>;

    /// Returns a value s.t. try_nth_root_of_unity(n) * try_nth_root_of_unity_inv(n) = 1
    fn try_nth_root_of_unity_inv(n: usize) -> Result<Self, Self::Error> {
        let mut root = Self::try_nth_root_of_unity(n)?;

        let mut result = <Self as MultiplicativeIdentity>::VALUE;
        let mut exp = n - 1;

        while exp > 0 {
            if exp % 2 == 1 {
                result = result * root;
            }

            root = root * root;
            exp >>= 1;
        }

        Ok(root)
    }
}
