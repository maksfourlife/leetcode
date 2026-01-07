use std::collections::{BTreeSet, HashMap};

fn group_anagrams<I>(strs: I) -> Vec<Vec<String>>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut groups = HashMap::<_, Vec<_>>::new();

    for s in strs.into_iter() {
        let s = s.as_ref().to_string();

        let mut chars: Vec<_> = s.chars().collect();
        chars.sort_unstable();

        groups.entry(chars).or_default().push(s);
    }

    groups.into_values().collect()
}

fn check(a: Vec<Vec<String>>, b: &[Vec<&str>]) {
    let a: BTreeSet<_> = a
        .iter()
        .map(|x| x.iter().map(|x| x.to_string()).collect::<BTreeSet<_>>())
        .collect();
    let b: BTreeSet<_> = b
        .iter()
        .map(|x| x.iter().map(|x| x.to_string()).collect::<BTreeSet<_>>())
        .collect();
    assert_eq!(a, b);
}

fn main() {
    check(
        group_anagrams(["eat", "tea", "tan", "ate", "nat", "bat"]),
        &[vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]],
    );
    check(group_anagrams([""]), &[vec![""]]);
    check(group_anagrams(["a"]), &[vec!["a"]]);
    check(
        group_anagrams(["abbbbbbbbbbb", "aaaaaaaaaaab"]),
        &[vec!["aaaaaaaaaaab"], vec!["abbbbbbbbbbb"]],
    );
}
