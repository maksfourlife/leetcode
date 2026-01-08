// https://leetcode.com/problems/maximum-subarray

#![allow(clippy::needless_range_loop)]

fn max_sub_array(nums: &[i32]) -> i32 {
    let (&first, rest) = nums.split_last().unwrap();
    rest.iter()
        .rev()
        .copied()
        .fold((first, first), |(sum, max), curr| {
            let next_sum = if sum < 0 { curr } else { sum + curr };
            (next_sum, std::cmp::max(next_sum, max))
        })
        .1
}

fn main() {
    assert_eq!(max_sub_array(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    assert_eq!(max_sub_array(&[1]), 1);
    assert_eq!(max_sub_array(&[5, 4, -1, 7, 8]), 23);
    assert_eq!(max_sub_array(&[-2, -1]), -1);
    assert_eq!(max_sub_array(&[-3, -2, -2, -3]), -2);
    assert_eq!(max_sub_array(&[1, 2]), 3);
}
