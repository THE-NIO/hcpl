use hcpl_complex::Complex;
use std::f64::consts::PI;

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

/// Performs the in-place Fast Fourier Transform on the slice `a`, whose lenght must be a power of two
pub fn fft(vec: &mut [Complex], inv: bool) {
    bit_reversal(vec);

    let n = vec.len();
    let sign = if inv { -1. } else { 1. };

    let mut width = 2;
    while width <= n {
        let w_d = Complex::cis(sign * 2. * PI / width as f64);

        for i in (0..n).step_by(width) {
            let mut w = Complex::ONE;
            for j in 0..width / 2 {
                let l = i + j;
                let r = i + j + width / 2;

                (vec[l], vec[r]) = (vec[l] + w * vec[r], vec[l] - w * vec[r]);
                w *= w_d;
            }
        }

        width *= 2;
    }

    if inv {
        for i in 0..n {
            vec[i] /= n as f64;
        }
    }
}

/// Performs the in-place discrete convolution of the two vectors `a` and `b`.
///
/// `a` will contain the convolution of `a` and `b`, and `b` will contain the discrete
/// Fourier transform of `b`.
pub fn convolution(a: &mut [Complex], b: &mut [Complex]) {
    fft(a, false);
    fft(b, false);
    for (x, y) in a.iter_mut().zip(b.iter().copied()) {
        *x = *x * y;
    }
    fft(a, true);
}
