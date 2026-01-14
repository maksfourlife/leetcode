// https://leetcode.com/problems/palindrome-partitioning

#![allow(clippy::ptr_arg)]

// use std::{collections::HashSet, ops::Range};

// fn partition(s: &str) -> Vec<Vec<String>> {
//     let mut output = vec![];
//     let orig = s.as_bytes();
//     f(orig, 0..s.len(), vec![], &mut HashSet::new(), &mut output);
//     output
//         .into_iter()
//         .map(|x| {
//             x.into_iter()
//                 .map(|r| unsafe { std::str::from_utf8_unchecked(&orig[r]) }.to_string())
//                 .collect::<Vec<_>>()
//         })
//         .collect()
// }

// fn f(
//     orig: &[u8],
//     range: Range<usize>,
//     curr: Vec<Range<usize>>,
//     is_palindrome: &mut HashSet<Range<usize>>,
//     output: &mut Vec<Vec<Range<usize>>>,
// ) {
//     let it = range.start + 1..range.end;

//     if it.is_empty() {
//         dbg!(&curr);
//     } else {
//         for i in it {
//             let left = range.start..i;
//             let right = i..range.end;
//             f(
//                 orig,
//                 right.clone(),
//                 [curr.clone(), vec![left.clone()]].concat(),
//                 is_palindrome,
//                 output,
//             );
//             f(
//                 orig,
//                 left,
//                 [curr.clone(), vec![right]].concat(),
//                 is_palindrome,
//                 output,
//             );
//         }
//     }
// }

use std::ops::Range;

fn f(orig: &[u8], range: Range<usize>) -> Vec<Vec<Vec<u8>>> {
    dbg!(&range);
    let it = range.start + 1..range.end;
    if it.is_empty() {
        vec![vec![vec![orig[range.start]]]]
    } else {
        it.flat_map(|i| {
            let left = f(orig, range.start..i);
            let right = f(orig, i..range.end);
            left.into_iter().flat_map(move |prefix| {
                right
                    .clone()
                    .into_iter()
                    .map(move |postfix| [prefix.clone(), postfix].concat())
            })
        })
        .collect()
    }
}

fn main() {
    let s = "aabcb";
    dbg!(f(s.as_bytes(), 0..s.len()));
}
