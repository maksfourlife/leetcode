// https://leetcode.com/problems/word-break/

#![allow(clippy::bool_assert_comparison)]

use std::{collections::HashMap, iter::Enumerate, str::Chars};

#[derive(Debug, Default)]
struct TrieNode {
    end: bool,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn insert<I>(&mut self, mut chars: I)
    where
        I: Iterator<Item = char>,
    {
        if let Some(ch) = chars.next() {
            self.children.entry(ch).or_default().insert(chars);
        } else {
            self.end = true;
        }
    }
}

fn word_break(s: &str, word_dict: &[String]) -> bool {
    let mut trie = TrieNode::default();
    word_dict.iter().for_each(|word| trie.insert(word.chars()));

    let mut mask = vec![false; s.len()];

    f(0, s.chars().enumerate(), &trie, &mut mask)
}

fn f(
    start_idx: usize,
    mut chars: Enumerate<Chars<'_>>,
    trie: &TrieNode,
    mask: &mut [bool],
) -> bool {
    if mask[start_idx] {
        return false;
    }
    let mut curr = trie;
    let mut end = curr.end;
    let mut next = vec![];
    while let Some((i, ch)) = chars.next() {
        let Some(next_curr) = curr.children.get(&ch) else {
            end = false;
            break;
        };
        curr = next_curr;
        end = curr.end;
        if curr.end {
            next.push((i, chars.clone()));
        }
    }
    let res = end
        || next
            .into_iter()
            .rev()
            .any(|(start_idx, chars)| f(start_idx, chars, trie, mask));
    mask[start_idx] = true;
    res
}

fn main() {
    assert_eq!(
        word_break("leetcode", &["leet".to_string(), "code".to_string()]),
        true
    );
    assert_eq!(
        word_break("applepenapple", &["apple".to_string(), "pen".to_string()]),
        true
    );
    assert_eq!(
        word_break(
            "catsandog",
            &[
                "cats".to_string(),
                "dog".to_string(),
                "sand".to_string(),
                "and".to_string(),
                "cat".to_string()
            ]
        ),
        false
    );
    assert_eq!(
        word_break("aaaaaaa", &["aaaa".to_string(), "aa".to_string()]),
        false
    );
    assert_eq!(
        word_break(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab",
            &[
                "a".to_string(),
                "aa".to_string(),
                "aaa".to_string(),
                "aaaa".to_string(),
                "aaaaa".to_string(),
                "aaaaaa".to_string(),
                "aaaaaaa".to_string(),
                "aaaaaaaa".to_string(),
                "aaaaaaaaa".to_string(),
                "aaaaaaaaaa".to_string()
            ]
        ),
        false
    );
}
