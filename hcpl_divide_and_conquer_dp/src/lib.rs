use hcpl_util::*;

#[allow(clippy::too_many_arguments)]
fn step<Cost, CostFinder>(
    l: usize,
    r: usize,
    optl: usize,
    optr: usize,
    get_cost: &mut CostFinder,
    max: &Cost,
    dp: &[Cost],
    new_dp: &mut Vec<Cost>,
) where
    CostFinder: FnMut(usize, usize) -> Cost,
    Cost: Ord + Clone + std::ops::Add<Output = Cost>,
{
    if l >= r {
        return;
    }

    let mid = l + (r - l) / 2;
    let mut opt_pair = (max.clone(), usize::MAX);

    for (i, val) in dp
        .iter()
        .enumerate()
        .take(std::cmp::min(optr, mid) + 1)
        .skip(optl)
    {
        opt_pair.set_min((val.clone() + get_cost(i, mid), i));
    }

    let (val, opt) = opt_pair;
    new_dp[mid] = val;

    step(l, mid, optl, opt, get_cost, max, dp, new_dp);
    step(mid + 1, r, opt, optr, get_cost, max, dp, new_dp);
}

pub fn solve<Cost, CostFinder>(n: usize, k: usize, mut get_cost: CostFinder) -> Cost
where
    CostFinder: FnMut(usize, usize) -> Cost,
    Cost: Ord + Clone + std::ops::Add<Output = Cost>,
{
    assert!(1 <= n);
    assert!(1 <= k);

    let max = get_cost(0, n);
    let mut dp: Vec<_> = (0..n + 1).map(|i| get_cost(0, i)).collect();
    let mut new_dp = vec![max.clone(); n + 1];

    for _ in 1..k {
        step(0, n + 1, 0, n + 1, &mut get_cost, &max, &dp, &mut new_dp);
        std::mem::swap(&mut dp, &mut new_dp);
    }

    dp.pop().unwrap()
}
