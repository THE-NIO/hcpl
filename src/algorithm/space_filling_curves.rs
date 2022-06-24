pub fn triangular(x: u32, y: u32, n: u32) -> u64 {
    debug_assert!(x < n);
    debug_assert!(y < n);
    debug_assert!(x <= y);

    if n == 1 {
        return 0;
    }

    let sml = (n - 1) / 2;
    let med = n / 2;
    let big = (n + 1) / 2;

    let med_area = med as u64 * (med as u64 + 1) / 2;
    let big_area = big as u64 * (big as u64 + 1) / 2;
    let square_area = big as u64 * big as u64;

    if y < med {
        // top
        return triangular(x, y, med);
    } else if x >= big {
        // right
        return triangular(x - big, y - big, med) + med_area + square_area;
    } else if x < n - y {
        // left
        return triangular(y - med, big - x - 1, big) + med_area;
    } else {
        // bottom
        return triangular(sml + med - y, x - 1, sml) + med_area + big_area;
    }
}

pub fn hilbert(bit: u32, mut x: u32, mut y: u32) -> u64 {
    let mut res = 0;
    let n = 1 << bit;
    let mut s = n >> 1;

    while s != 0 {
        let rotate_x = if x & s != 0 { 1 } else { 0 };
        let rotate_y = if y & s != 0 { 1 } else { 0 };

        res += s as u64 * s as u64 * ((rotate_x) ^ (rotate_y * 3));

        s >>= 1;

        if rotate_x != 0 {
            continue;
        }

        if rotate_y != 0 {
            x = n - 1 - x;
            y = n - 1 - y;
        }

        std::mem::swap(&mut x, &mut y);
    }

    res
}
