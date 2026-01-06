// https://leetcode.com/problems/add-two-numbers

#![allow(clippy::boxed_local)]

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

fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut out = None::<Box<ListNode>>;
    let mut curr = &mut out;

    let mut carry = 0;

    while l1.is_some() || l2.is_some() || carry > 0 {
        let mut x = carry;

        if let Some(node) = l1 {
            x += node.val;
            l1 = node.next;
        }

        if let Some(node) = l2 {
            x += node.val;
            l2 = node.next;
        }

        let val = x % 10;
        carry = x / 10;

        if let Some(curr) = curr {
            curr.next = Some(Box::new(ListNode { val, next: None }));
        } else {
            *curr = Some(Box::new(ListNode { val, next: None }));
        }
        curr = &mut curr.as_mut().unwrap().next;
    }

    out
}

fn main() {
    let l1 = Some(Box::new(make_list![2, 4, 3]));
    let l2 = Some(Box::new(make_list![5, 6, 4]));
    dbg!(add_two_numbers(l1, l2));

    let l1 = Some(Box::new(make_list![0]));
    let l2 = Some(Box::new(make_list![0]));
    dbg!(add_two_numbers(l1, l2));

    let l1 = Some(Box::new(make_list![9, 9, 9, 9, 9, 9, 9]));
    let l2 = Some(Box::new(make_list![9, 9, 9, 9]));
    dbg!(add_two_numbers(l1, l2));
}
