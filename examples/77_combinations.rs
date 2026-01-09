// https://leetcode.com/problems/combinations/
// Given two integers n and k, return all possible combinations of k numbers chosen from the range [1, n].

use std::collections::HashMap;

fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    f(n, k, 1, &mut HashMap::new())
}

fn f(n: i32, k: i32, s: i32, cache: &mut HashMap<(i32, i32), Vec<Vec<i32>>>) -> Vec<Vec<i32>> {
    if let Some(res) = cache.get(&(k, s)) {
        return res.clone();
    }
    if k == 0 {
        vec![vec![]]
    } else {
        let res: Vec<_> = (s..=n)
            .flat_map(|x| {
                f(n, k - 1, x + 1, cache)
                    .into_iter()
                    .map(move |y| [vec![x], y].concat())
            })
            .collect();
        cache.insert((k, s), res.clone());
        res
    }
}

fn main() {
    assert_eq!(
        combine(4, 2),
        [[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]
    );
    assert_eq!(combine(1, 1), [[1]]);
}
