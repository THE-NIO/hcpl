use std::cell::Cell;

/// A union find data structure implemented with path compression
/// and union-by-size.
pub struct UnionFind {
    data: Vec<Cell<i32>>,
}

impl UnionFind {
    /// Creates a new Union-Find collection with `n` nodes
    pub fn new(n: usize) -> Self {
        Self {
            data: vec![Cell::new(-1); n],
        }
    }

    /// Returns the representative for the set containing `i`
    pub fn find(&self, i: usize) -> usize {
        if self.data[i].get() < 0 {
            i
        } else {
            let ans = self.find(self.data[i].get() as usize);
            self.data[i].set(ans as i32);
            ans
        }
    }

    /// Unites the sets containing `i` and `j`.
    ///
    /// If `i` and `j` are already elements of the same set, this function returns
    /// `None`. Otherwise, it returns `Some((new_root, old_root))`.
    pub fn unite(&mut self, mut i: usize, mut j: usize) -> Option<(usize, usize)> {
        i = self.find(i);
        j = self.find(j);
        if i == j {
            None
        } else {
            if -self.data[i].get() < -self.data[j].get() {
                std::mem::swap(&mut i, &mut j);
            }
            self.data[i].set(self.data[i].get() + self.data[j].get());
            self.data[j].set(i as i32);
            Some((i, j))
        }
    }

    /// Returns the size of the set containing `i`.
    pub fn cardinality(&self, i: usize) -> usize {
        -self.data[self.find(i)].get() as usize
    }
}
