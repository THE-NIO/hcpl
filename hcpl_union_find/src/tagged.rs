use hcpl_algebra::Monoid;

/// A Union-Find data structure using path compression and
/// union-by-size.
pub struct UnionFind<V: Monoid> {
    // NOTE: `isize` is large enough to store any valid index becuase
    // the size of a `Vec<T>` is bounded by `isize::MAX` if `T` is not
    // a ZST.
    parent_map: Vec<isize>,
    data: Vec<Option<V>>,
}

impl<V: Monoid> UnionFind<V> {
    /// Creates a new Union-Find collection with `n` nodes
    pub fn new(n: usize) -> Self {
        Self {
            parent_map: vec![-1; n],
            data: (0..n).map(|_| Some(V::IDENTITY)).collect(),
        }
    }

    /// Creates a new empty Union-Find collection
    pub fn empty() -> Self {
        Self {
            parent_map: Vec::new(),
            data: Vec::new(),
        }
    }

    /// Returns the size of this Union-Find collection
    pub fn len(&self) -> usize {
        self.parent_map.len()
    }

    /// Adds a new node to the Union-Find collection.
    pub fn push(&mut self, v: V) -> usize {
        self.parent_map.push(-1);
        self.data.push(Some(v));

        self.parent_map.len() - 1
    }

    /// Returns the representative for the set containing `i`
    pub fn find(&mut self, i: usize) -> usize {
        if self.parent_map[i] < 0 {
            i
        } else {
            let ans = self.find(self.parent_map[i] as usize);
            self.parent_map[i] = ans as isize;
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
            let new_data = V::op(self.data[i].take().unwrap(), self.data[j].take().unwrap());
            if -self.parent_map[i] < -self.parent_map[j] {
                std::mem::swap(&mut i, &mut j);
            }
            self.parent_map[i] += self.parent_map[j];
            self.parent_map[j] = i as isize;
            self.data[i] = Some(new_data);
            Some((i, j))
        }
    }

    /// Returns the size of the set containing `i`.
    pub fn cardinality(&mut self, i: usize) -> usize {
        let root = self.find(i);
        -self.parent_map[root] as usize
    }

    /// Returns an immutable reference to the value associated with the set containing
    /// `i`.
    pub fn get(&mut self, i: usize) -> &V {
        let root = self.find(i);
        self.data[root].as_ref().unwrap()
    }

    /// Returns a mutable reference to the value associated with the set containing
    /// `i`.
    pub fn get_mut(&mut self, i: usize) -> &mut V {
        let root = self.find(i);
        self.data[root].as_mut().unwrap()
    }
}

impl<V: Monoid> Extend<V> for UnionFind<V> {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = V>,
    {
        let iter = iter.into_iter();

        let (lower_bound, _) = iter.size_hint();

        self.parent_map.reserve(lower_bound);
        self.data.reserve(lower_bound);

        for item in iter {
            self.parent_map.push(-1);
            self.data.push(Some(item));
        }
    }
}
