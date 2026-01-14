// https://leetcode.com/problems/string-to-integer-atoi

fn my_atoi(s: &str) -> i32 {
    const DIGITS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut it = s.chars().peekable();

    // skip leading whitespace
    while it.next_if_eq(&' ').is_some() {}

    let mut is_negative = false;

    match it.peek() {
        Some('-') => {
            is_negative = true;
            it.next();
        }
        Some('+') => {
            it.next();
        }
        _ => {}
    }

    // skip leading zeros
    while it.next_if_eq(&'0').is_some() {}

    let mut x = 0i64;

    for ch in it {
        match DIGITS.binary_search(&ch) {
            Ok(index) => {
                x = x.saturating_mul(10).saturating_add(index as i64);
            }
            Err(_) => break,
        }
    }

    if is_negative {
        x *= -1;
    }

    x.clamp(i32::MIN as i64, i32::MAX as i64) as i32
}

fn main() {
    assert_eq!(my_atoi("42"), 42);
    assert_eq!(my_atoi(" -042"), -42);
    assert_eq!(my_atoi("1337c0d3"), 1337);
    assert_eq!(my_atoi("0-1"), 0);
    assert_eq!(my_atoi("words and 987"), 0);
    assert_eq!(my_atoi("9223372036854775808"), 2147483647);
    assert_eq!(my_atoi("-91283472332"), -2147483648);
}
