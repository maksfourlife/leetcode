// https://leetcode.com/problems/rotate-list/

use leetcode_rust::{ListNode, make_list};

fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut head = head?;

    // if n=1 k does not matter
    if head.next.is_none() {
        return Some(head);
    }

    let mut curr = head.next.as_mut().unwrap();
    let mut n = 2;

    while curr.next.is_some() {
        curr = curr.next.as_mut().unwrap();
        n += 1;
    }

    if k % n == 0 {
        return Some(head);
    }

    let mut curr = &mut head;

    for _ in 0..n - (k % n) - 1 {
        curr = curr.next.as_mut().unwrap();
    }

    // curr is last before disjoint node

    let mut first = curr.next.take();

    let mut curr = first.as_mut().unwrap();

    while curr.next.is_some() {
        curr = curr.next.as_mut().unwrap();
    }

    curr.next = Some(head);

    first
}

fn main() {
    assert_eq!(
        rotate_right(Some(Box::new(make_list![1, 2, 3, 4, 5])), 2),
        Some(Box::new(make_list![4, 5, 1, 2, 3]))
    );

    assert_eq!(
        rotate_right(Some(Box::new(make_list![0, 1, 2])), 4),
        Some(Box::new(make_list![2, 0, 1]))
    );

    assert_eq!(
        rotate_right(Some(Box::new(make_list![1, 2])), 0),
        Some(Box::new(make_list![1, 2]))
    );

    assert_eq!(
        rotate_right(Some(Box::new(make_list![1, 2])), 1),
        Some(Box::new(make_list![2, 1]))
    );
}
