// https://leetcode.com/problems/merge-intervals/

#![allow(clippy::single_range_in_vec_init)]

use std::ops::Range;

// fn merge_orig(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
//     let x = intervals
//         .into_iter()
//         .map(|x| x[0] as usize..x[1] as usize)
//         .collect();
//     let x = merge(x);
//     x.into_iter()
//         .map(|x| vec![x.start as i32, x.end as i32])
//         .collect()
// }

fn merge(mut intervals: Vec<Range<usize>>) -> Vec<Range<usize>> {
    intervals.sort_unstable_by_key(|x| x.start);

    let mut it = intervals.into_iter();

    let mut prev = it.next().unwrap();
    let mut out = vec![];

    for curr in it {
        if curr.start <= prev.end {
            if curr.end > prev.end {
                prev.end = curr.end;
            }
        } else {
            out.push(prev);
            prev = curr;
        }
    }

    out.push(prev);

    out
}

fn main() {
    assert_eq!(
        merge(vec![1..3, 2..6, 8..10, 15..18]),
        [1..6, 8..10, 15..18]
    );
    assert_eq!(merge(vec![1..4, 4..5]), [1..5]);
    assert_eq!(merge(vec![4..7, 1..4]), [1..7]);
    assert_eq!(merge(vec![1..4, 2..3]), [1..4]);
}
