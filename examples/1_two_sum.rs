// https://leetcode.com/problems/two-sum

use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut c = HashMap::<_, Vec<_>>::with_capacity(nums.len());
    nums.iter()
        .enumerate()
        .for_each(|(i, num)| c.entry(*num).or_default().push(i));

    // dbg!(&c);

    for (x, indices1) in &c {
        let i = indices1[0];
        if let Some(indices2) = c.get(&(target - *x))
            && let Some(j) = indices2.iter().find(|j| j != &&i)
        {
            return vec![i as i32, *j as i32];
        }
    }

    panic!();
}

fn main() {
    dbg!(two_sum(vec![3, 3], 6));
}
