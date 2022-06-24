use crate::algebra::Ring;
use std::{fmt::Debug, ops::DivAssign};

/// In-place Fast Walsh–Hadamard Transform of slice a'''
pub fn fwht<T: Copy + Ring + DivAssign<T> + TryFrom<usize>>(a: &mut [T], inv: bool)
where
    <T as TryFrom<usize>>::Error: Debug,
{
    let n = a.len();
    let mut step = 1;
    while step < n {
        for i in (0..n).step_by(2 * step) {
            for j in i..i + step {
                let u = a[j];
                let v = a[j + step];
                a[j] = u + v;
                a[j + step] = u - v;
            }
        }
        step <<= 1;
    }

    if inv {
        let d = T::try_from(n).unwrap();
        for x in a.iter_mut() {
            *x /= d;
        }
    }
}

pub fn xor_convolution<T: Copy + Ring + DivAssign<T> + TryFrom<usize>>(a: &mut [T], mut b: Vec<T>)
where
    <T as TryFrom<usize>>::Error: Debug,
{
    fwht(a, false);
    fwht(&mut b, false);
    for (x, y) in a.iter_mut().zip(b) {
        *x = *x * y;
    }
    fwht(a, true);
}
