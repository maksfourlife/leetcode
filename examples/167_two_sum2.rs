// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted

use std::cmp::Ordering;

fn two_sum(nums: &[i32], target: i32) -> Vec<i32> {
    let mut r = 0..(nums.len() - 1);

    while !r.is_empty() {
        let curr = nums[r.start] + nums[r.end];

        match curr.cmp(&target) {
            Ordering::Equal => {
                return vec![r.start as i32 + 1, r.end as i32 + 1];
            }
            Ordering::Less => {
                r.start += 1;
            }
            Ordering::Greater => {
                r.end -= 1;
            }
        }
    }

    panic!()
}

fn main() {
    dbg!(two_sum(&[2, 7, 11, 15], 9));
    dbg!(two_sum(&[2, 3, 4], 6));
    dbg!(two_sum(&[-1, 0], -1));
    dbg!(two_sum(&[-5, -3, 0, 2, 4, 6, 8], 5));
}
