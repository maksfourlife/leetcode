// https://leetcode.com/problems/subsets/

fn subsets(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut out = vec![];

    let mut prev: Vec<Vec<i32>> = vec![vec![]];
    let mut curr: Vec<Vec<i32>> = vec![];

    while prev[0].len() < nums.len() {
        prev.iter().for_each(|prev_item| {
            curr.extend(
                nums.iter()
                    .filter(|&num| !matches!(prev_item.last(), Some(x) if x >= num))
                    .map(|&num| [prev_item.to_vec(), vec![num]].concat()),
            );
        });
        out.extend(prev);
        prev = curr;
        curr = vec![];
    }

    out.extend(prev);

    out
}

fn main() {
    assert_eq!(
        subsets(&[1, 2, 3]),
        &[
            vec![],
            vec![1,],
            vec![2,],
            vec![3,],
            vec![1, 2,],
            vec![1, 3,],
            vec![2, 3,],
            vec![1, 2, 3,]
        ]
    );
    assert_eq!(subsets(&[0]), &[vec![], vec![0]])
}
