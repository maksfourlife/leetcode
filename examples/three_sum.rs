use std::collections::{BTreeMap, HashSet};

fn three_sum(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut c = BTreeMap::<_, usize>::new();
    nums.iter().for_each(|&num| *c.entry(num).or_default() += 1);

    dbg!(&c);

    let mut out = HashSet::new();

    if c.get(&0) == Some(&3) {
        out.insert([0, 0, 0]);
    }

    c.iter().for_each(|(num1, num1_count)| {
        if *num1_count >= 2 {
            let x = -(num1 + num1);
            if c.contains_key(&x) {
                let mut t = [*num1, *num1, x];
                t.sort();
                out.insert(t);
            }
        }
        c.iter()
            .filter(|(num2, _)| num1 != *num2)
            .for_each(|(num2, _)| {
                // dbg!((num1, num2));
                let x = -(num1 + num2);
                if c.contains_key(&x) {
                    let mut t = [*num1, *num2, x];
                    t.sort();
                    out.insert(t);
                }
            })
    });

    out.into_iter().map(|x| x.to_vec()).collect()
}

fn main() {
    let a = [-1, 0, 1, 2, -1, -4];
    let b = three_sum(&a);
    dbg!(b);
}
