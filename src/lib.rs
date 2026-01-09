#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

#[macro_export]
macro_rules! make_list {
    [$first:expr, $($rest:expr),+ $(,)?] => {
        ListNode {
            next: Some(Box::new(make_list![$($rest),+])),
            val: $first,
        }
    };
    [$val:expr] => {
        ListNode {
            next: None,
            val: $val,
        }
    };
}
