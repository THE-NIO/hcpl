// Ported from https://github.com/kth-competitive-programming/kactl/blob/main/content/number-theory/Factor.h

use super::gcd;

fn modmul(x: u64, y: u64, m: u64) -> u64 {
    (x as u128 * y as u128 % m as u128) as u64
}

fn modpow(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut a = 1;
    while e != 0 {
        if e & 1 == 1 {
            a = modmul(a, b, m);
        }
        b = modmul(b, b, m);
        e >>= 1;
    }
    a
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 || n % 6 % 4 != 1 {
        return (n | 1) == 3;
    }

    let a = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];
    let s = (n - 1).trailing_zeros();
    let d = n >> s;

    for x in a {
        let mut p = modpow(x % n, d, n);
        let mut i = s;
        while p != 1 && p != n - 1 && x % n != 0 && i != 0 {
            i -= 1;
            p = modmul(p, p, n);
        }
        if p != n - 1 && i != s {
            return false;
        }
    }

    true
}

pub fn pollard(n: u64) -> u64 {
    let f = |x: u64| modmul(x, x, n) + 1;
    let mut x = 0;
    let mut y = 0;
    let mut t = 30;
    let mut prd = 2;
    let mut i = 1;

    while t % 40 != 0 || gcd(prd, n) == 1 {
        t += 1;
        if x == y {
            i += 1;
            x = i;
            y = f(x);
        }

        let q = modmul(prd, std::cmp::max(x, y) - std::cmp::min(x, y), n);
        if q != 0 {
            prd = q;
        }
        x = f(x);
        y = f(f(y));
    }
    gcd(prd, n)
}

pub fn factor(n: u64) -> Vec<u64> {
    if n == 1 {
        return Vec::new();
    }
    if is_prime(n) {
        return vec![n];
    }
    let x = pollard(n);
    let mut l = factor(x);
    let mut r = factor(n / x);
    l.append(&mut r);
    l
}
