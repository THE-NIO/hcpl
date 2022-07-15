use hcpl_integer::Integer;

pub fn gcd<T: Integer>(su: T, sv: T) -> T::AsUnsigned
where
    T::AsUnsigned: Integer,
{
    let mut u = su.unsigned_abs();
    let mut v = sv.unsigned_abs();

    use std::cmp::min;
    use std::mem::swap;

    if u.is_zero() {
        return v;
    } else if v.is_zero() {
        return u;
    }

    let i = u.trailing_zeros();
    let j = v.trailing_zeros();
    u >>= i;
    v >>= j;

    let k = min(i, j);

    loop {
        if u > v {
            swap(&mut u, &mut v);
        }

        v -= u;

        if v.is_zero() {
            return u << k;
        }

        v >>= v.trailing_zeros();
    }
}
