// https://leetcode.com/problems/decode-ways

#![allow(clippy::while_let_on_iterator)]

fn num_decodings(s: &str) -> i32 {
    let mut chars = s.chars().peekable();

    let mut prev = None;

    let mut a = 0;
    let mut b = 1;

    while let Some(curr) = chars.next() {
        if curr == '0' {
            return 0;
        }

        let is_next_zero = chars.next_if_eq(&'0').is_some();

        if is_next_zero && !(curr == '1' || curr == '2') {
            return 0;
        }

        let can_concat = !is_next_zero
            && matches!(
                (prev, curr),
                (Some('1'), '0'..='9') | (Some('2'), '0'..='6')
            );

        (a, b) = (b, if can_concat { a + b } else { b });

        prev = Some(if is_next_zero { '0' } else { curr });
    }

    b
}

fn main() {
    assert_eq!(num_decodings("1011"), 2);
    assert_eq!(num_decodings("12"), 2);
    assert_eq!(num_decodings("226"), 3);
    assert_eq!(num_decodings("227"), 2);
    assert_eq!(num_decodings("06"), 0);
    assert_eq!(num_decodings("230"), 0);
}
