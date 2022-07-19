use std::{fmt::Debug, ops::RangeBounds};

const INNER_SIZE: usize = 32;
type Snapshot = u32;

#[derive(Copy, Clone)]
struct Inner<T> {
    snapshots: [Snapshot; INNER_SIZE],
    _marker: std::marker::PhantomData<T>,
}

impl<T: Ord + Clone + Debug> Inner<T> {
    fn new() -> Self {
        Self {
            snapshots: [0; INNER_SIZE],
            _marker: std::marker::PhantomData,
        }
    }

    fn init(&mut self, data: &[T]) -> u32 {
        let mut stack: Snapshot = 0;
        for i in 0..INNER_SIZE.min(data.len()) {
            while stack != 0 {
                let last = std::mem::size_of::<Snapshot>() * 8 - stack.leading_zeros() as usize - 1;
                if data[i] > data[last] {
                    break;
                }
                stack ^= 1 << last;
            }
            stack ^= 1 << i;
            self.snapshots[i] = stack;
        }
        self.argmin(0, INNER_SIZE.min(data.len()) as u32)
    }

    fn argmin(&self, from: u32, to: u32) -> u32 {
        let pre = self.snapshots[to as usize - 1];
        let pre_suf = pre & !((1 << from) - 1);
        pre_suf.trailing_zeros()
    }
}

pub struct RMQ<'a, T> {
    table: Vec<Vec<u32>>,
    inners: Vec<Inner<T>>,
    data: &'a [T],
}

impl<'a, T: Ord + Clone + Debug> RMQ<'a, T> {
    pub fn new(data: &'a [T]) -> Self {
        let n = data.len();
        let blocks = (n + INNER_SIZE - 1) / INNER_SIZE;
        let lvls = std::mem::size_of::<usize>() * 8 - blocks.leading_zeros() as usize;

        let mut res = Self {
            table: vec![Vec::new(); lvls],
            inners: vec![Inner::new(); blocks],
            data,
        };

        res.table[0].reserve(blocks);
        for block in 0..blocks {
            let begin = block * INNER_SIZE;
            let end = (begin + INNER_SIZE).min(n);
            res.table[0].push(begin as u32 + res.inners[block].init(&data[begin..end]));
        }

        let mut width = 1;
        for lvl in 1..lvls {
            res.table[lvl].reserve(blocks - 2 * width + 1);
            for i in 0..=blocks - 2 * width {
                let min = res.select(res.table[lvl - 1][i], res.table[lvl - 1][i + width]);
                res.table[lvl].push(min);
            }
            width *= 2;
        }

        res
    }

    pub fn argmin(&self, range: impl RangeBounds<usize>) -> usize {
        use std::ops::Bound::*;

        let i = match range.start_bound() {
            Included(&n) => n,
            Excluded(&n) => n + 1,
            Unbounded => 0,
        };
        let j = match range.end_bound() {
            Included(&n) => n + 1,
            Excluded(&n) => n,
            Unbounded => self.data.len(),
        };

        assert!(i < j);

        let i_block = i / INNER_SIZE;
        let j_block = (j - 1) / INNER_SIZE;
        let i_offs = i % INNER_SIZE;
        let j_offs = (j - 1) % INNER_SIZE + 1;

        if i_block == j_block {
            self.inner_argmin(i_block, i_offs, j_offs) as usize
        } else if i_block + 1 == j_block {
            self.select(
                self.inner_argmin(i_block, i_offs, INNER_SIZE),
                self.inner_argmin(j_block, 0, j_offs),
            ) as usize
        } else {
            let lvl = std::mem::size_of::<usize>() * 8
                - (j_block - i_block - 1).leading_zeros() as usize
                - 1;
            let w = 1 << lvl;

            self.select(
                self.select(
                    self.inner_argmin(i_block, i_offs, INNER_SIZE),
                    self.inner_argmin(j_block, 0, j_offs),
                ),
                self.select(self.table[lvl][i_block + 1], self.table[lvl][j_block - w]),
            ) as usize
        }
    }

    pub fn min(&self, range: impl RangeBounds<usize>) -> &T {
        &self.data[self.argmin(range)]
    }

    fn inner_argmin(&self, block: usize, i: usize, j: usize) -> u32 {
        let res = self.inners[block].argmin(i as u32, j as u32);
        (block * INNER_SIZE) as u32 + res
    }

    fn select(&self, i: u32, j: u32) -> u32 {
        if self.data[i as usize] < self.data[j as usize] {
            i
        } else {
            j
        }
    }
}
