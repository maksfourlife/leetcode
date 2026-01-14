// https://leetcode.com/problems/search-a-2d-matrix

fn search_matrix<T>(mut matrix: &[Vec<T>], target: &T) -> bool
where
    T: PartialOrd + PartialEq,
{
    if matrix[0].is_empty() {
        return false;
    }

    while matrix.len() > 1 {
        let mid = matrix.len() / 2;
        matrix = if target < &matrix[mid][0] {
            &matrix[..mid]
        } else {
            &matrix[mid..]
        };
    }

    let mut slice = &matrix[0][..];

    while slice.len() > 1 {
        let mid = slice.len() / 2;
        slice = if target < &slice[mid] {
            &slice[..mid]
        } else {
            &slice[mid..]
        };
    }

    &slice[0] == target
}

fn main() {
    assert!(search_matrix(
        &[vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
        &3
    ));
    assert!(!search_matrix(
        &[vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
        &13
    ));
}
