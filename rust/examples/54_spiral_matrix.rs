// https://leetcode.com/problems/spiral-matrix

#![allow(clippy::needless_range_loop)]

fn spiral_order(matrix: &[Vec<i32>]) -> Vec<i32> {
    let m = matrix.len();
    let n = matrix[0].len();

    let k = std::cmp::min(m, n) / 2;

    let mut out = vec![];

    for d in 0..k {
        for i in d..n - d - 1 {
            out.push(matrix[d][i]);
        }

        for i in d..m - d - 1 {
            out.push(matrix[i][n - d - 1]);
        }

        for i in (d + 1..n - d).rev() {
            out.push(matrix[m - d - 1][i]);
        }

        for i in (d + 1..m - d).rev() {
            out.push(matrix[i][d]);
        }
    }

    if m <= n && m % 2 == 1 {
        for i in k..n - k {
            out.push(matrix[k][i]);
        }
    } else if n <= m && n % 2 == 1 {
        for i in k..m - k {
            out.push(matrix[i][k]);
        }
    }

    out
}

fn main() {
    assert_eq!(
        spiral_order(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        [1, 2, 3, 6, 9, 8, 7, 4, 5]
    );

    assert_eq!(
        spiral_order(&[
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16]
        ]),
        [1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10]
    );

    assert_eq!(
        spiral_order(&[vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]]),
        [1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
    );
}
