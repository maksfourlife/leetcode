// https://leetcode.com/problems/triangle
//
// Given a triangle array, return the minimum path sum from top to bottom.
//
// For each step, you may move to an adjacent number of the row below. More formally, if you are on index i on the current row, you may move to either index i or index i + 1 on the next row.

fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
    for i in (0..triangle.len() - 1).rev() {
        for j in 0..triangle[i].len() {
            triangle[i][j] += std::cmp::min(triangle[i + 1][j], triangle[i + 1][j + 1]);
        }
    }
    triangle[0][0]
}

fn main() {
    assert_eq!(
        minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
        11
    );
    assert_eq!(minimum_total(vec![vec![-10]]), -10);
}
