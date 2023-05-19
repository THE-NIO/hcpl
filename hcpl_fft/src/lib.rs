use hcpl_algebra::monoid::MultiplicativeIdentity;
use hcpl_number_theory::roots::TryNthRootOfUnity;
use std::fmt::Debug;

/// Performs the bit-reversal permutation on `a`
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

mod stack_stack {
    use std::mem::MaybeUninit;

    const BUF_SIZE: usize = 100;

    /// A stack-allocated stack
    pub(crate) struct StackStack<T> {
        buffer: [MaybeUninit<T>; BUF_SIZE],
        len: usize,
    }

    impl<T> StackStack<T> {
        pub(crate) fn new() -> Self {
            Self {
                buffer: unsafe {
                    MaybeUninit::<[MaybeUninit<T>; BUF_SIZE]>::uninit().assume_init()
                },
                len: 0,
            }
        }

        pub(crate) fn push(&mut self, val: T) -> Option<()> {
            if self.len == BUF_SIZE {
                return None;
            }

            self.buffer[self.len].write(val);
            self.len += 1;

            Some(())
        }

        pub(crate) fn pop(&mut self) -> Option<T> {
            if self.len == 0 {
                return None;
            }

            self.len -= 1;

            // SAFETY: this element will have been written to exactly once, and won't be read again
            Some(unsafe { self.buffer[self.len].assume_init_read() })
        }
    }
}

/// Performs the in-place Fast Fourier Transform on the slice `a`, whose lenght must be a power of two. DOES NOT
/// PERFORM NORMALISATION
pub fn fft<T>(vec: &mut [T], inv: bool)
where
    T: TryNthRootOfUnity,
    <T as TryNthRootOfUnity>::Error: Debug,
{
    bit_reversal(vec);

    let n = vec.len();

    let mut roots = stack_stack::StackStack::new();

    let mut q = n;
    let mut last: T = if inv {
        TryNthRootOfUnity::try_nth_root_of_unity_inv(n)
    } else {
        TryNthRootOfUnity::try_nth_root_of_unity(n)
    }
    .unwrap();
    while q >= 2 {
        roots.push(last);
        q /= 2;
        last = last * last;
    }

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
