use std::fmt::Debug;

pub struct Tensor<T, const D: usize> {
    inner: Vec<T>,
    dims: [usize; D],
}

impl<T, const D: usize> Tensor<T, D> {
    pub fn new(x: T, dims: [usize; D]) -> Self
    where
        T: Clone,
    {
        Self {
            inner: vec![x; dims.iter().product()],
            dims,
        }
    }

    pub fn from_vec(v: Vec<T>, dims: [usize; D]) -> Self {
        assert_eq!(dims.iter().product::<usize>(), v.len());
        Self { inner: v, dims }
    }

    pub fn from_iter_and_dims<It: Iterator<Item = T>>(iter: It, dims: [usize; D]) -> Self {
        let size = dims.iter().product();
        let inner: Vec<_> = iter.take(size).collect();
        assert_eq!(inner.len(), size);
        Self { inner, dims }
    }

    fn index_pos(&self, index: [usize; D]) -> usize {
        let mut pos = 0;
        for (s, i) in self.dims.iter().zip(index) {
            pos *= s;
            pos += i;
        }
        pos
    }
}

impl<T, const D: usize> std::ops::Index<[usize; D]> for Tensor<T, D> {
    type Output = T;

    fn index(&self, index: [usize; D]) -> &Self::Output {
        let pos = self.index_pos(index);
        &self.inner[pos]
    }
}

impl<T, const D: usize> std::ops::IndexMut<[usize; D]> for Tensor<T, D> {
    fn index_mut(&mut self, index: [usize; D]) -> &mut Self::Output {
        let pos = self.index_pos(index);
        &mut self.inner[pos]
    }
}

impl<T: Debug> Debug for Tensor<T, 2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Tensor{:?} {{\n", &self.dims))?;
        for i in 0..self.dims[0] {
            let begin = i * self.dims[1];
            let end = begin + self.dims[1];
            f.write_fmt(format_args!("    {:?}\n", &self.inner[begin..end]))?;
        }
        f.write_str("}")?;
        Ok(())
    }
}
