pub trait State {
    type Output;

    fn insert(&mut self, i: u32);
    fn erase(&mut self, i: u32);
    fn get(&self) -> Self::Output;
}

pub fn solve<S: State>(n: u32, queries: &[(u32, u32)], mut state: S) -> Vec<S::Output> {
    let mut indices: Vec<usize> = (0..queries.len()).collect();
    indices.sort_by_cached_key(|&i| {
        let (x, y1) = queries[i];
        triangle_order(x, y1 - 1, n)
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

/// Triangular space filling curve for sorting queries
/// with small sum of Manhattan distances.
///
/// Based on splitting an axis aligned triangle into 4 parts,
/// finding the order inside each smaller triangle,
/// and joining up the orders so endpoints meet close by.
/// ```
/// use hcpl_mo::triangle_order;
///
/// let order = [
///     vec![0],
///     vec![1,2],
///
///
///     vec![6,5,3],
///     vec![7,4,    11,   12],
///     vec![8,    9,10,   13,14],
/// ];
///
/// for y in 0..order.len() {
///     for x in 0..order[y].len() {
///         assert_eq!(order[y][x], triangle_order(x as u32, y as u32, order.len() as u32));
///     }
/// }
/// ```
///
/// Sorting this way ensures O(n sqrt q) insertions and deletions in Mo's algorithm,
/// without needing to pick any bucket size.
pub fn triangle_order(x: u32, y: u32, n: u32) -> u64 {
    debug_assert!(x < n);
    debug_assert!(y < n);
    debug_assert!(x <= y);

    if n == 1 {
        return 0;
    }

    let sml = (n - 1) / 2;
    let med = n / 2;
    let big = (n + 1) / 2;

    let med_area = med as u64 * (med as u64 + 1) / 2;
    let big_area = big as u64 * (big as u64 + 1) / 2;
    let square_area = big as u64 * big as u64;

    if y < med {
        // top
        triangle_order(x, y, med)
    } else if x >= big {
        // right
        triangle_order(x - big, y - big, med) + med_area + square_area
    } else if x < n - y {
        // left
        triangle_order(y - med, big - x - 1, big) + med_area
    } else {
        // bottom
        triangle_order(sml + med - y, x - 1, sml) + med_area + big_area
    }
}
