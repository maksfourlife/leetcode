// https://leetcode.com/problems/spiral-matrix/description/

#![allow(clippy::needless_range_loop)]

fn spiral_order(matrix: &[Vec<i32>]) -> Vec<i32> {
    let m = matrix.len();
    let n = matrix[0].len();

    let mut out = vec![];

    for i in 0..std::cmp::min(m.div_ceil(2), n.div_ceil(2)) {
        dbg!(i);
        for k in i..(n - 1 - i) {
            out.push(matrix[i][k]);
        }
        // for k in i..(m - 1 - i) {
        //     out.push(matrix[k][m - 1 - j]);
        // }
    }

    out
}

fn main() {
    dbg!();
    let a = [vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let b = spiral_order(&a);
    dbg!(&b);

    // let a = [vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
    // let b = spiral_order(&a);
    // dbg!(&b);
}
