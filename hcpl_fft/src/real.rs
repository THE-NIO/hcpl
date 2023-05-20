//! FFTs of real-valued data
//!
//! Functions in this module should act the same as functions in the crate root but may assume that the input or output is real-valued.
//! 
//! PLEASE DON'T RELY ON THIS AT THE MOMENT AS IT LACKS TESTING

use std::f64::consts::PI;

use hcpl_complex::Complex;

use crate::fft;

/// Performs both recursive calls in the Cooley-Tukey algorithm with only one call to `fft`. `vec` must be real-valued.
pub fn double_real_ffft(vec: &mut [Complex]) {
    let n = vec.len() / 2;

    for i in 0..n {
        vec[i] = Complex {
            re: vec[2 * i].re,
            im: vec[2 * i + 1].re,
        };
    }

    fft(&mut vec[..n], false);

    vec.copy_within(..n, n);
    vec[n + 1..].reverse();

    // To see why this works, consider the case where all imaginary components are zero. The FFT famously exhibits conjugate symmetry
    // on real data, meaning that vec[-i] = vec[i]*. Therefore `z_c` (= vec[-i]* = vec[i]** = vec[i]) is equal to `z`, and
    // `(z + z_c) / 2 == z` and `-i * (z - z_c) / 2 == 0`. These `z` values are simply the DHT of the input array, as expected.
    //
    // The same argument works if all real components are zero, except the output will have been multiplied by `i`. Using linearity one
    // can show that vec[..n] will contain the DHT of the even-indexed values in the input and vec[n..] will contain the DHT of the
    // odd-indexed values in the input.
    for i in 0..n {
        let z = vec[i];
        let z_c = vec[i + n].conj();

        vec[i] = (z + z_c) / 2.;
        vec[i + n] = -Complex::I * (z - z_c) / 2.;
    }
}

/// Calculates the FFT of a sequence of real numbers
pub fn real_ffft(vec: &mut [Complex]) {
    let n = vec.len();

    double_real_ffft(vec);

    let mut w = Complex::ONE;
    let w_d = Complex::cis(2. * PI / n as f64);
    for i in 0..n / 2 {
        (vec[i], vec[i + n / 2]) = (vec[i] + w * vec[i + n / 2], vec[i] - w * vec[i + n / 2]);
        w *= w_d;
    }
}

// These functions were written by manually inverting the functions above.

/// Inverse of double_real_fft, up to normalisation
pub fn double_real_ifft(vec: &mut [Complex]) {
    // a, b = (x + y)/2, (x - y) / 2i
    // a + bi = x
    // a - bi = y

    let n = vec.len() / 2;

    for i in 0..n {
        vec[i] = vec[i] + Complex::I * vec[i + n];
    }

    fft(&mut vec[..n], true);

    for i in (0..n).rev() {
        (vec[2 * i], vec[2 * i + 1]) = (vec[i].re.into(), vec[i].im.into());
    }
}

/// Calculates the IFFT of a sequence which is the DFT of a sequence of real numbers. Does NOT perform normalisation.
pub fn real_ifft(vec: &mut [Complex]) {
    // a, b = x + w y, x - w y
    // a + b = x + w y + x - w y = 2 x
    // a - b = x + w y - x + w y = 2 w y
    // (a + b) / 2 = x
    // (a - b) / 2w = y

    let n = vec.len();

    let mut w = Complex::ONE;
    let w_d = Complex::cis(-2. * PI / n as f64);

    for i in 0..n/2 {
        (vec[i], vec[i + n / 2]) = ((vec[i] + vec[i + n / 2]) / 2., w * (vec[i] - vec[i + n / 2]) / 2.);
        w *= w_d;
    }

    double_real_ifft(vec);

    // apply missing denormalisation
    for i in 0..n {
        vec[i] *= 2.;
    }
}

/// Like real_ffft and real_ifft but using an extra parameter to choose direction
pub fn real_fft(vec: &mut [Complex], inverse: bool) {
    if inverse {
        real_ifft(vec);
    } else {
        real_ffft(vec);
    }
}
