// https://leetcode.com/problems/simplify-path

use std::ops::Range;

fn simplify_path(path: &str) -> String {
    let mut parts = vec![];
    let mut curr = None::<Range<usize>>;

    fn f<'a>(parts: &mut Vec<&'a str>, curr: Option<Range<usize>>, path: &'a str) {
        if let Some(curr) = curr {
            let curr = &path[curr];
            if curr == ".." {
                parts.pop();
            } else if curr != "." {
                parts.push(curr);
            }
        }
    }

    for (i, ch) in path.chars().enumerate() {
        if ch == '/' {
            f(&mut parts, curr, path);
            curr = None;
        } else if let Some(curr) = &mut curr {
            curr.end = i + 1;
        } else {
            curr = Some(i..i + 1)
        }
    }

    f(&mut parts, curr, path);

    "/".to_string() + &parts.join("/")
}

fn main() {
    // assert_eq!(simplify_path("/home/"), "/home");
    // assert_eq!(simplify_path("/home//foo/"), "/home/foo");
    // assert_eq!(
    //     simplify_path("/home/user/Documents/../Pictures"),
    //     "/home/user/Pictures"
    // );
    // assert_eq!(simplify_path("/../"), "/");
    // assert_eq!(simplify_path("/.../a/../b/c/../d/./"), "/.../b/d");
    assert_eq!(simplify_path("/a//b////c/d//././/.."), "/a/b/c");
}
