// https://leetcode.com/problems/add-two-numbers

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

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

fn add_two_numbers(mut l1: Box<ListNode>, mut l2: Box<ListNode>) -> Option<Box<ListNode>> {
    let l1_len = list_len(&l1);
    let l2_len = list_len(&l2);

    dbg!(l1_len, l2_len);

    let mut out = None;

    for _ in l2_len..l1_len {
        dbg!();
        let val;
        (val, l1) = (l1.val, l1.next.unwrap());
        out = Some(Box::new(ListNode { val, next: out }));
    }

    for _ in l1_len..l2_len {
        dbg!();
        let val;
        (val, l2) = (l2.val, l2.next.unwrap());
        out = Some(Box::new(ListNode { val, next: out }));
    }

    dbg!(&out);

    let mut carry = 0;

    for _ in 0..(std::cmp::min(l1_len, l2_len) - 1) {
        dbg!();
        let l1_val;
        let l2_val;
        (l1_val, l1) = (l1.val, l1.next.unwrap());
        (l2_val, l2) = (l2.val, l2.next.unwrap());
        let x = carry + l1_val + l2_val;
        out = Some(Box::new(ListNode {
            val: x % 10,
            next: out,
        }));
        carry = x / 10;
    }

    let x = carry + l1.val + l2.val;
    out = Some(Box::new(ListNode {
        val: x % 10,
        next: out,
    }));

    let carry = x / 10;
    if carry > 0 {
        out = Some(Box::new(ListNode {
            val: carry,
            next: out,
        }));
    }

    out
}

fn list_len(l: &ListNode) -> usize {
    let mut curr = l;
    let mut len = 1;
    while curr.next.is_some() {
        curr = curr.next.as_ref().unwrap();
        len += 1;
    }
    len
}

fn main() {
    // let l1 = Box::new(make_list![2, 4, 3]);
    // let l2 = Box::new(make_list![5, 6, 4]);
    // dbg!(add_two_numbers(l1, l2));

    // let l1 = Box::new(make_list![0]);
    // let l2 = Box::new(make_list![0]);
    // dbg!(add_two_numbers(l1, l2));

    let l1 = Box::new(make_list![9, 9, 9, 9, 9, 9, 9]);
    let l2 = Box::new(make_list![9, 9, 9, 9]);
    dbg!(add_two_numbers(l1, l2));
}
