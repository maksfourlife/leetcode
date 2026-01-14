// https://leetcode.com/problems/combination-sum

use std::{
    cmp::Ordering,
    iter::{empty, once},
};

fn combination_sum(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
    candidates
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(i, candidate)| match candidate.cmp(&target) {
            Ordering::Less => Box::new(
                combination_sum(&candidates[i..], target - candidate)
                    .into_iter()
                    .map(move |res| [vec![candidate], res].concat()),
            ) as Box<dyn Iterator<Item = Vec<i32>>>,
            Ordering::Equal => Box::new(once(vec![candidate])),
            Ordering::Greater => Box::new(empty()),
        })
        .collect()
}

fn main() {
    assert_eq!(combination_sum(&[2, 3, 6, 7], 7), [vec![2, 2, 3], vec![7]]);
    assert_eq!(
        combination_sum(&[2, 3, 5], 8),
        [vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
    );
    assert_eq!(combination_sum(&[2], 1), [] as [Vec<_>; 0]);
}
