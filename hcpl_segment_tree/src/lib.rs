use hcpl_algebra::monoid::Monoid;
use std::{iter::FromIterator, ops::RangeBounds};

pub use hcpl_algebra::monoid;

#[derive(Clone)]
/// A segment tree with values of type `T`
pub struct SegmentTree<T: Monoid + Clone> {
    n: usize,
    pub values: Vec<T>,
}

impl<T: Monoid + Clone> SegmentTree<T> {
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

    /// Creates a segment tree from a slice.
    pub fn new(src: &[T]) -> Self {
        Self::new_inner(src.len(), |v| v.extend_from_slice(src))
    }

    /// Creates a segment tree initialised with `T::IDENTITY`.
    pub fn with_size(size: usize) -> Self {
        SegmentTree {
            n: size,
            values: vec![T::IDENTITY; size.next_power_of_two() * 2],
        }
    }
}

impl<T: Monoid + Clone> FromIterator<T> for SegmentTree<T> {
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

/// A mutable reference to a leaf node in a [`SegmentTree`]
pub struct PointReferenceMut<'a, T: Monoid + Clone> {
    st: &'a mut SegmentTree<T>,
    i: usize,
}

impl<'a, T: Monoid + Clone> std::ops::Deref for PointReferenceMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.st.values[self.i]
    }
}

impl<'a, T: Monoid + Clone> std::ops::DerefMut for PointReferenceMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.st.values[self.i]
    }
}

impl<'a, T: Monoid + Clone> Drop for PointReferenceMut<'a, T> {
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

impl<T: Monoid + Clone> SegmentTree<T> {
    fn pull(&mut self, i: usize) {
        debug_assert!(i < self.offset());
        self.values[i] = T::op(self.values[2 * i].clone(), self.values[2 * i + 1].clone());
    }

    fn offset(&self) -> usize {
        self.values.len() / 2
    }

    /// Returns a mutable reference to the leaf node with the given `index`.
    pub fn get_mut(&mut self, index: usize) -> PointReferenceMut<'_, T> {
        debug_assert!(index < self.n);

        let offset = self.offset();
        PointReferenceMut {
            st: self,
            i: index + offset,
        }
    }

    /// Returns an immutable reference to the leaf node with the given `index`.
    pub fn get(&self, index: usize) -> &T {
        debug_assert!(index < self.n);
        &self.values[index + self.offset()]
    }

    /// Replaces `tree[index]` with `T::op(tree[index], value)`.
    pub fn add(&mut self, mut index: usize, value: &T) {
        index += self.offset();
        self.values[index] = T::op(self.values[index].clone(), (*value).clone());

        for i in parent_chain(index) {
            self.values[i] = T::op(self.values[i].clone(), value.clone());
        }
    }

    /// Returns the monoid fold of all the values in the given `range`.
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
                l = T::op(l, self.values[i + 1].clone());
            }
            if j & 1 == 1 {
                r = T::op(self.values[j - 1].clone(), r);
            }

            i >>= 1;
            j >>= 1;
        }

        T::op(l, r)
    }

    pub fn right_while<P: Fn(&T) -> bool>(&self, start: usize, predicate: P) -> (usize, T) {
        debug_assert!(start <= self.n);
        debug_assert!(predicate(&T::IDENTITY));

        if start == self.n {
            return (start, T::IDENTITY);
        }

        let offset = self.offset();
        let mut i = start + offset;
        let mut sum = T::IDENTITY;
        let mut nxt_sum;

        loop {
            i >>= i.trailing_zeros();

            nxt_sum = T::op(sum.clone(), self.values[i].clone());

            if !predicate(&nxt_sum) {
                while i < offset {
                    i <<= 1;
                    nxt_sum = T::op(sum.clone(), self.values[i].clone());
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

impl<T: Monoid + Clone + std::fmt::Debug> std::fmt::Debug for SegmentTree<T> {
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
