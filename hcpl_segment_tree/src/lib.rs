use std::{iter::FromIterator, ops::RangeBounds};

pub use hcpl_algebra::monoid;

pub struct SegmentTree<T: monoid::Monoid> {
    n: usize,
    pub values: Vec<T>,
}

impl<T: monoid::Monoid + Clone> SegmentTree<T> {
    fn new_inner<F: FnOnce(&mut Vec<T>)>(n: usize, f: F) -> Self {
        let offset = n.next_power_of_two();
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

    pub fn with_size(size: usize) -> Self {
        SegmentTree {
            n: size,
            values: vec![T::IDENTITY; size.next_power_of_two() * 2],
        }
    }
}

impl<T: monoid::Monoid + Clone> FromIterator<T> for SegmentTree<T> {
    fn from_iter<It: IntoIterator<Item = T>>(iter: It) -> Self {
        let iter = iter.into_iter();
        let (lower, upper) = iter.size_hint();
        if Some(lower) == upper {
            Self::new_inner(lower, |v| v.extend(iter))
        } else {
            let all_items: Vec<_> = iter.collect();
            Self::new(&all_items)
        }
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
        for idx in parent_chain(self.i) {
            self.st.pull(idx);
        }
    }
}

fn try_parent(idx: usize) -> Option<usize> {
    match idx / 2 {
        0 => None,
        n => Some(n),
    }
}

fn parent_chain(idx: usize) -> impl Iterator<Item = usize> {
    std::iter::successors(try_parent(idx), |&idx| try_parent(idx))
}

impl<T: monoid::Monoid> SegmentTree<T> {
    fn pull(&mut self, i: usize) {
        debug_assert!(i < self.values.len() / 2);
        self.values[i] = T::op(&self.values[2 * i], &self.values[2 * i + 1]);
    }

    fn offset(&self) -> usize {
        self.values.len() / 2
    }

    pub fn get_mut<'a>(&'a mut self, index: usize) -> PointReferenceMut<'a, T> {
        debug_assert!(index < self.n);
        let offset = self.offset();
        PointReferenceMut {
            st: self,
            i: index + offset,
        }
    }

    pub fn get<'a>(&'a self, index: usize) -> &T {
        debug_assert!(index < self.n);
        &self.values[index + self.values.len() / 2]
    }

    pub fn fold<R>(&self, range: R) -> T
    where
        R: RangeBounds<usize>,
    {
        use std::ops::Bound::*;

        let start = match range.start_bound() {
            Included(&n) => n,
            Excluded(&n) => n + 1,
            Unbounded => 0,
        };
        let end = match range.end_bound() {
            Included(&n) => n + 1,
            Excluded(&n) => n,
            Unbounded => self.n,
        };

        if start > end {
            return T::IDENTITY;
        }

        debug_assert!(end <= self.n);
        let offset = self.offset();

        let mut i = start + offset - 1;
        let mut j = end + offset;

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
        struct Layers<'a, T>(&'a [T]);

        impl<'a, T: std::fmt::Debug> std::fmt::Debug for Layers<'a, T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut debug_list = f.debug_list();

                let mut begin = 1;
                while begin < self.0.len() {
                    debug_list.entry(&&self.0[begin..begin << 1]);
                    begin <<= 1;
                }

                debug_list.finish()
            }
        }

        f.debug_struct("SegmentTree")
            .field("layers", &Layers(&self.values))
            .finish()
    }
}
