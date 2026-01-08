// https://leetcode.com/problems/unique-paths

fn unique_paths(m: usize, n: usize) -> i32 {
    if m == 1 && n == 1 {
        return 1;
    }
    let mut mat = vec![vec![0i32; n]; m];
    if n > 1 {
        mat[0][1] = 1;
    }
    if m > 1 {
        mat[1][0] = 1;
    }
    for i in 0..m {
        for j in 0..n {
            if i > 0 {
                mat[i][j] += mat[i - 1][j];
            }
            if j > 0 {
                mat[i][j] += mat[i][j - 1];
            }
        }
    }
    mat[m - 1][n - 1]
}

fn main() {
    assert_eq!(unique_paths(3, 7), 28);
    assert_eq!(unique_paths(3, 2), 3);
    assert_eq!(unique_paths(1, 2), 1);
}
