// https://leetcode.com/problems/valid-sudoku/

#![allow(clippy::needless_range_loop)]

fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    const DIGITS: &[char] = &['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    for i in 0..9 {
        let mut mask = 0u16;
        for j in 0..9 {
            let ch = &board[i][j];
            if let Ok(idx) = DIGITS.binary_search(ch) {
                if mask & (1 << idx) != 0 {
                    return false;
                }
                mask |= 1 << idx;
            }
        }
    }

    for i in 0..9 {
        let mut mask = 0u16;
        for j in 0..9 {
            let ch = &board[j][i];
            if let Ok(idx) = DIGITS.binary_search(ch) {
                if mask & (1 << idx) != 0 {
                    return false;
                }
                mask |= 1 << idx;
            }
        }
    }

    for k in 0..9 {
        let mut mask = 0u16;
        let h = k / 3;
        let w = k % 3;
        for i in h * 3..(h + 1) * 3 {
            for j in w * 3..(w + 1) * 3 {
                let ch = &board[i][j];
                if let Ok(idx) = DIGITS.binary_search(ch) {
                    if mask & (1 << idx) != 0 {
                        return false;
                    }
                    mask |= 1 << idx;
                }
            }
        }
    }

    true
}

fn main() {
    let board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    assert!(is_valid_sudoku(board));

    let board = vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    assert!(!is_valid_sudoku(board));

    let board = vec![
        vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
        vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
        vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
        vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
        vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
        vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
        vec!['.', '.', '4', '.', '.', '.', '.', '.', '.'],
    ];
    assert!(!is_valid_sudoku(board));
}
