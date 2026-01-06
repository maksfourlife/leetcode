// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array

fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    step(&nums, &target, 0, 0)
        .map(|x| vec![x.start as i32, x.end as i32 - 1])
        .unwrap_or_else(|| vec![-1, -1])
}

use std::{cmp::Ordering, ops::Range};

fn step<T>(orig: &[T], target: &T, offset: usize, _d: usize) -> Option<Range<usize>>
where
    T: Ord + std::fmt::Debug,
{
    // dbg!(d, orig);

    if orig.is_empty() {
        return None;
    }
    if orig.len() == 1 {
        return if &orig[0] == target {
            Some(offset..offset + 1)
        } else {
            None
        };
    }

    let pos = orig.len() / 2;

    let mid = &orig[pos];
    let left = &orig[..pos];
    let right = &orig[pos..];

    match mid.cmp(target) {
        Ordering::Equal => {
            let left = step(left, target, offset, _d + 1);
            let right = step(right, target, offset + pos, _d + 1);
            // dbg!(pos, &left, &right);
            Some(
                left.map(|x| x.start).unwrap_or(offset + pos)
                    ..right.map(|x| x.end).unwrap_or(offset + pos),
            )
        }
        Ordering::Greater => step(left, target, offset, _d + 1),
        Ordering::Less => step(right, target, offset + pos, _d + 1),
    }
}

fn main() {
    assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
    assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
    assert_eq!(search_range(vec![], 0), vec![-1, -1]);
    assert_eq!(search_range(vec![2, 2], 2), vec![0, 1]);
    assert_eq!(search_range(vec![1, 2, 2, 2, 3], 2), vec![1, 3]);
    assert_eq!(search_range(vec![1, 2, 2, 3], 2), vec![1, 2]);
    assert_eq!(search_range(vec![1, 2, 3], 3), vec![2, 2]);
}
