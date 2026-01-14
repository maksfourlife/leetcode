// https://leetcode.com/problems/set-matrix-zeroes

#![allow(clippy::needless_range_loop)]

fn set_zeroes(matrix: &mut [Vec<i32>]) {
    let m = matrix.len();
    let n = matrix[0].len();

    let mut r = vec![];
    let mut c = vec![];

    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == 0 {
                r.push(i);
                c.push(j);
            }
        }
    }

    for i in r {
        for j in 0..n {
            matrix[i][j] = 0;
        }
    }

    for i in c {
        for j in 0..m {
            matrix[j][i] = 0;
        }
    }
}

fn main() {
    // let mut matrix = [vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    // set_zeroes(&mut matrix);
    // assert_eq!(matrix, [[1, 0, 1], [0, 0, 0], [1, 0, 1]]);

    let mut matrix = [vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
    set_zeroes(&mut matrix);
    assert_eq!(matrix, [[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]]);
}
