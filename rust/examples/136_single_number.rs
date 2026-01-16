// https://leetcode.com/problems/single-number

fn single_number(nums: &[i32]) -> i32 {
    let (&first, nums) = nums.split_first().unwrap();
    nums.iter().fold(first, |acc, curr| acc ^ curr)
}

fn main() {
    assert_eq!(single_number(&[2, 2, 1]), 1);
    assert_eq!(single_number(&[4, 1, 2, 1, 2]), 4);
    assert_eq!(single_number(&[1]), 1);
}
