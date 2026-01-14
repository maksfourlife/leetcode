// https://leetcode.com/problems/permutations

fn permute(nums: Vec<i32>, k: usize) -> Vec<Vec<i32>> {
    let r = k..nums.len();
    if r.is_empty() {
        vec![nums]
    } else {
        r.flat_map(|i| {
            let mut nums2 = nums.clone();
            nums2.swap(k, i);
            permute(nums2, k + 1)
        })
        .collect()
    }
}

fn main() {
    assert_eq!(
        permute(vec![1, 2, 3], 0),
        [
            [1, 2, 3],
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 2, 1],
            [3, 1, 2]
        ]
    );
    assert_eq!(permute(vec![0, 1], 0), [[0, 1], [1, 0]]);
    assert_eq!(permute(vec![1], 0), [[1]]);
}
