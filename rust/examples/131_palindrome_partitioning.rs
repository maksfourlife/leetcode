// https://leetcode.com/problems/palindrome-partitioning

#![allow(clippy::ptr_arg)]

use std::{collections::HashSet, ops::Range};

fn partition(s: &str) -> Vec<Vec<String>> {
    let palindrome_set = palindrome_set(s);

    let mut palindromes: Vec<_> = palindrome_set.into_iter().collect();
    palindromes.sort_unstable_by_key(|x| x.start);

    let mut out = vec![];
    f(&palindromes, vec![], &mut out, s.len());

    let bytes = s.as_bytes();

    out.into_iter()
        .map(|ranges| {
            ranges
                .into_iter()
                .map(|range| unsafe { std::str::from_utf8_unchecked(&bytes[range]) }.to_string())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn f(
    ranges: &[Range<usize>],
    curr: Vec<Range<usize>>,
    out: &mut Vec<Vec<Range<usize>>>,
    target_len: usize,
) {
    ranges
        .iter()
        .enumerate()
        .filter(|(_, new_range)| new_range.start == curr.last().map(|x| x.end).unwrap_or(0))
        .for_each(|(i, new_range)| {
            let mut new_curr = curr.clone();
            new_curr.push(new_range.clone());
            f(&ranges[i + 1..], new_curr, out, target_len);
        });
    if let Some(last) = curr.last()
        && last.end == target_len
    {
        out.push(curr);
    }
}

fn palindrome_set(s: &str) -> HashSet<Range<usize>> {
    let s = s.as_bytes();

    let mut is_palindrome = HashSet::with_capacity(s.len() * (1 + s.len()) / 2);

    for i in 0..s.len() {
        is_palindrome.insert(i..i + 1);
    }

    for k in 1..=s.len() {
        for i in 0..s.len() - k + 1 {
            let curr = i..i + k;
            let prev = curr.start + 1..curr.end - 1;

            if (prev.is_empty() || is_palindrome.contains(&prev))
                && s[curr.start] == s[curr.end - 1]
            {
                is_palindrome.insert(curr);
            }
        }
    }

    is_palindrome
}

fn main() {
    assert_eq!(partition("a"), [["a"]]);
    dbg!(partition("aab"));
    dbg!(partition("aabcb"));
}
