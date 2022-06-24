pub trait State {
    type Output;

    fn insert(&mut self, i: u32);
    fn erase(&mut self, i: u32);
    fn get(&self) -> Self::Output;
}

pub fn solve<S: State>(n: u32, queries: &Vec<(u32, u32)>, mut state: S) -> Vec<S::Output> {
    let mut pow = 1;
    while 1 << pow < n {
        pow += 1;
    }

    let mut indices: Vec<usize> = (0..queries.len()).collect();
    indices.sort_by_cached_key(|&i| {
        let (x, y1) = queries[i];
        super::space_filling_curves::hilbert(pow, x, y1 - 1)
    });

    let mut answer = Vec::<Option<S::Output>>::with_capacity(queries.len());
    for _ in 0..queries.len() {
        answer.push(None);
    }

    let mut l = 0;
    let mut r = 0;

    for i in indices {
        let (target_l, target_r) = queries[i];
        while r < target_r {
            state.insert(r);
            r += 1;
        }
        while l > target_l {
            l -= 1;
            state.insert(l);
        }
        while r > target_r {
            r -= 1;
            state.erase(r);
        }
        while l < target_l {
            state.erase(l);
            l += 1;
        }
        answer[i] = Some(state.get());
    }

    answer.into_iter().map(|a| a.unwrap()).collect()
}
