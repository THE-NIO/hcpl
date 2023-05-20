#[cfg(feature = "real")]
pub mod real;

mod stack_stack;

use hcpl_algebra::monoid::MultiplicativeIdentity;
use hcpl_number_theory::roots::TryNthRootOfUnity;
use std::fmt::Debug;

/// Performs the bit-reversal permutation on `vec`
pub fn bit_reversal<T>(vec: &mut [T]) {
    let n = vec.len();

    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while (j & bit) != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;

        if i < j {
            vec.swap(i, j);
        }
    }
}

fn get_roots<T>(mut n: usize, inv: bool) -> stack_stack::StackStack<T>
where
    T: TryNthRootOfUnity,
    <T as TryNthRootOfUnity>::Error: Debug,
{
    let mut roots = stack_stack::StackStack::new();

    let mut last: T = if inv {
        TryNthRootOfUnity::try_nth_root_of_unity_inv(n)
    } else {
        TryNthRootOfUnity::try_nth_root_of_unity(n)
    }
    .unwrap();
    while n >= 2 {
        roots.push(last);
        n /= 2;
        last = last * last;
    }

    roots
}

/// Performs the in-place Fast Fourier Transform on the slice `vec`, whose lenght must be a power of two. DOES NOT
/// PERFORM NORMALISATION
pub fn fft<T>(vec: &mut [T], inv: bool)
where
    T: TryNthRootOfUnity,
    <T as TryNthRootOfUnity>::Error: Debug,
{
    bit_reversal(vec);

    let n = vec.len();

    let mut roots = get_roots(n, inv);
    let mut width = 2;
    while width <= n {
        let w_d = roots.pop().unwrap();

        for i in (0..n).step_by(width) {
            let mut w = <T as MultiplicativeIdentity>::VALUE;
            for j in 0..width / 2 {
                let l = i + j;
                let r = i + j + width / 2;

                (vec[l], vec[r]) = (vec[l] + w * vec[r], vec[l] - w * vec[r]);
                w = w * w_d;
            }
        }

        width *= 2;
    }
}

/// Performs the in-place discrete convolution of the two vectors `a` and `b`. DOES NOT PERFORM
/// NORMALISATION
///
/// `a` will contain the convolution of `a` and `b`, and `b` will contain the discrete
/// Fourier transform of `b`.
pub fn convolution<T>(a: &mut [T], b: &mut [T])
where
    T: TryNthRootOfUnity,
    <T as TryNthRootOfUnity>::Error: Debug,
{
    fft(a, false);
    fft(b, false);
    for (x, y) in a.iter_mut().zip(b.iter().copied()) {
        *x = *x * y;
    }
    fft(a, true);
}
