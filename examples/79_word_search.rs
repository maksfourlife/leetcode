// https://leetcode.com/problems/word-search

#![allow(clippy::bool_assert_comparison)]

use std::{iter::Peekable, str::Chars};

fn exist(board: &[Vec<char>], word: &str) -> bool {
    let m = board.len();
    let n = board[0].len();
    let mask = vec![vec![false; n]; m];
    for i in 0..m {
        for j in 0..n {
            let mut mask = mask.clone();
            mask[i][j] = true;
            if f(board, &mask, word.chars().peekable(), i, j) {
                return true;
            }
        }
    }
    false
}

fn f(
    board: &[Vec<char>],
    mask: &[Vec<bool>],
    mut chars: Peekable<Chars<'_>>,
    i: usize,
    j: usize,
) -> bool {
    // println!("i = {i}, j = {j}, {mask:?}");

    let m = board.len();
    let n = board[0].len();

    let Some(ch) = chars.next() else {
        return true;
    };

    if board[i][j] != ch {
        return false;
    }

    if chars.peek().is_none() {
        return true;
    }

    let mut cords = [None; 4];

    if j > 0 {
        cords[0] = Some((i, j - 1));
    }
    if j < n - 1 {
        cords[1] = Some((i, j + 1));
    }
    if i > 0 {
        cords[2] = Some((i - 1, j));
    }
    if i < m - 1 {
        cords[3] = Some((i + 1, j));
    }

    for cord in cords {
        if let Some((i, j)) = cord
            && !mask[i][j]
        {
            let mut mask = mask.to_vec();
            mask[i][j] = true;
            if f(board, &mask, chars.clone(), i, j) {
                return true;
            }
        }
    }

    false
}

fn main() {
    assert_eq!(
        exist(
            &[
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCCED"
        ),
        true
    );

    assert_eq!(
        exist(
            &[
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "SEE"
        ),
        true
    );

    assert_eq!(
        exist(
            &[
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCB"
        ),
        false
    );

    assert_eq!(exist(&[vec!['a']], "a"), true);

    assert_eq!(exist(&[vec!['a', 'a']], "aaa"), false);
}
