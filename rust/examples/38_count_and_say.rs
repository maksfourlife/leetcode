// https://leetcode.com/problems/count-and-say

fn count_and_say(n: i32) -> String {
    let mut a = "1".to_string();

    for _ in 0..n - 1 {
        let mut b = "".to_string();

        let mut it = a.chars();

        let mut count = 1;
        let mut prev = it.next().unwrap();

        for ch in it {
            if ch == prev {
                count += 1;
            } else {
                b += &format!("{count}{prev}");
                prev = ch;
                count = 1;
            }
        }

        a = b + &format!("{count}{prev}");
    }

    a
}

fn main() {
    assert_eq!(count_and_say(4), "1211");
    assert_eq!(count_and_say(1), "1");
}
