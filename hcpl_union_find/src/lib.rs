use std::cell::Cell;

pub struct UnionFind {
    data: Vec<Cell<i32>>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            data: vec![Cell::new(-1); n],
        }
    }

    pub fn find(&self, i: usize) -> usize {
        if self.data[i].get() < 0 {
            i
        } else {
            let ans = self.find(self.data[i].get() as usize);
            self.data[i].set(ans as i32);
            ans
        }
    }

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
}
