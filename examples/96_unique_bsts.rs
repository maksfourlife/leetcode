fn num_trees(n: usize) -> usize {
    let mut num_trees = vec![1; n + 1];
    (2..n + 1).for_each(|k| {
        num_trees[k] = (1..=k)
            .map(|i| num_trees[i - 1] * num_trees[k - i])
            .sum::<usize>();
    });
    num_trees[n]
}

fn main() {
    assert_eq!(num_trees(0), 1);
    assert_eq!(num_trees(1), 1);
    assert_eq!(num_trees(2), 2);
    assert_eq!(num_trees(3), 5);
    assert_eq!(num_trees(5), 42);
}
