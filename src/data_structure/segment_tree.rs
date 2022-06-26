use std::ops::Range;

pub use crate::algebra::monoid;

fn up_to_pow2(n: usize) -> usize {
    let mut res = 1;
    while res < n {
        res <<= 1;
    }
    res
}

pub struct SegmentTree<T: monoid::Monoid> {
    n: usize,
    pub values: Vec<T>,
}

impl<T: monoid::Monoid + Clone> SegmentTree<T> {
    fn new_inner<F: FnOnce(&mut Vec<T>)>(n: usize, f: F) -> Self {
        let offset = up_to_pow2(n);
        let mut values = Vec::with_capacity(2 * offset);
        values.extend(std::iter::repeat(T::IDENTITY).take(offset));
        f(&mut values);
        debug_assert!(values.len() <= 2 * offset);
        values.extend(std::iter::repeat(T::IDENTITY).take(2 * offset - values.len()));
        let mut res = Self { n, values };
        for i in (1..offset).rev() {
            res.pull(i);
        }
        res
    }

    pub fn new(src: &[T]) -> Self {
        Self::new_inner(src.len(), |v| v.extend_from_slice(src))
    }
}

impl<It: std::iter::ExactSizeIterator<Item = T>, T: monoid::Monoid + Clone> From<It>
    for SegmentTree<T>
{
    fn from(it: It) -> Self {
        Self::new_inner(it.len(), |v| v.extend(it))
    }
}

pub struct PointReferenceMut<'a, T: monoid::Monoid> {
    st: &'a mut SegmentTree<T>,
    i: usize,
}

impl<'a, T: monoid::Monoid> std::ops::Deref for PointReferenceMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.st.values[self.i]
    }
}

impl<'a, T: monoid::Monoid> std::ops::DerefMut for PointReferenceMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.st.values[self.i]
    }
}

impl<'a, T: monoid::Monoid> Drop for PointReferenceMut<'a, T> {
    fn drop(&mut self) {
        while {
            self.i /= 2;
            self.i > 0
        } {
            self.st.pull(self.i);
        }
    }
}

impl<T: monoid::Monoid> SegmentTree<T> {
    fn pull(&mut self, i: usize) {
        debug_assert!(i < self.values.len() / 2);
        self.values[i] = T::op(&self.values[2 * i], &self.values[2 * i + 1]);
    }

    pub fn get_mut<'a>(&'a mut self, index: usize) -> PointReferenceMut<'a, T> {
        debug_assert!(index < self.n);
        let offset = self.values.len() / 2;
        PointReferenceMut {
            st: self,
            i: index + offset,
        }
    }

    pub fn get<'a>(&'a self, index: usize) -> &T {
        debug_assert!(index < self.n);
        &self.values[index + self.values.len() / 2]
    }

    pub fn fold(&self, index: Range<usize>) -> T {
        if index.start > index.end {
            return T::IDENTITY;
        }

        debug_assert!(index.end <= self.n);
        let offset = self.values.len() / 2;

        let mut i = index.start + offset - 1;
        let mut j = index.end + offset;

        let mut l = T::IDENTITY;
        let mut r = T::IDENTITY;

        while i + 1 < j {
            if i & 1 == 0 {
                l = T::op(&l, &self.values[i + 1]);
            }
            if j & 1 == 1 {
                r = T::op(&self.values[j - 1], &r);
            }

            i >>= 1;
            j >>= 1;
        }

        T::op(&l, &r)
    }

    pub fn right_while<P: Fn(&T) -> bool>(&self, start: usize, predicate: P) -> (usize, T) {
        debug_assert!(start <= self.n);
        debug_assert!(predicate(&T::IDENTITY));

        if start == self.n {
            return (start, T::IDENTITY);
        }

        let offset = self.values.len() / 2;
        let mut i = start + offset;
        let mut sum = T::IDENTITY;
        let mut nxt_sum;

        loop {
            i >>= i.trailing_zeros();

            nxt_sum = T::op(&sum, &self.values[i]);

            if !predicate(&nxt_sum) {
                while i < offset {
                    i <<= 1;
                    nxt_sum = T::op(&sum, &self.values[i]);
                    if predicate(&nxt_sum) {
                        sum = nxt_sum;
                        i += 1;
                    }
                }
                // it's ok to go beyond n and take min after
                // because [n,...) are only T::IDENTITY
                return (std::cmp::min(i - offset, self.n), sum);
            }

            sum = nxt_sum;
            i += 1;

            if (i & (i - 1)) == 0 {
                break;
            }
        }

        (self.n, sum)
    }
}

impl<T: monoid::Monoid + std::fmt::Debug> std::fmt::Debug for SegmentTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SegmentTree{")?;

        let mut begin = 1;
        while begin < self.values.len() {
            f.debug_list()
                .entries(&self.values[begin..begin << 1])
                .finish()?;
            begin <<= 1;
        }

        f.write_str("}")
    }
}
