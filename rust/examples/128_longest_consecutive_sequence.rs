// https://leetcode.com/problems/longest-consecutive-sequence/

use std::collections::HashSet;

fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut s = HashSet::with_capacity(nums.len());
    s.extend(nums);

    let mut max_len = 0;

    while let Some(&start) = s.iter().next() {
        s.remove(&start);

        let mut left = start - 1;
        while s.contains(&left) {
            left -= 1;
        }

        let mut right = start + 1;
        while s.contains(&right) {
            right += 1;
        }

        max_len = std::cmp::max(max_len, right - left - 1);
    }

    max_len
}

fn main() {
    assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    assert_eq!(longest_consecutive(vec![1, 0, 1, 2]), 3);
}
