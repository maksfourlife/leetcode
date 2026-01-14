// https://leetcode.com/problems/longest-palindromic-substring/

use std::{collections::HashSet, ops::Range};

fn longest_palindrome(s: &str) -> &str {
    if s.len() <= 1 {
        return s;
    }

    let mut is_palindrome = HashSet::<Range<usize>>::with_capacity(s.len().pow(2) / 2);

    let mut best = 0..1;

    // s consist of only digits and English letters
    let s = s.as_bytes();

    for k in 1..=s.len() {
        for i in 0..s.len() - k + 1 {
            let curr = i..i + k;
            let prev = curr.start + 1..curr.end - 1;

            if (prev.len() <= 1 || is_palindrome.contains(&prev))
                && s[curr.start] == s[curr.end - 1]
            {
                if curr.len() > best.len() {
                    best = curr.clone();
                }
                is_palindrome.insert(curr);
            }
        }
    }

    unsafe { std::str::from_utf8_unchecked(&s[best.start..best.end]) }
}

fn main() {
    dbg!(longest_palindrome("a"));
    dbg!(longest_palindrome("ac"));
    dbg!(longest_palindrome("bb"));
    dbg!(longest_palindrome("babad"));
    dbg!(longest_palindrome("cbbd"));
    dbg!(longest_palindrome("ccbbbcd"));
}
