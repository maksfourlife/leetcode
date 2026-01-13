// https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree

#![allow(unused)]

use std::{cell::RefCell, rc::Rc};

use leetcode::{ListNode, make_list};

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut curr = head?;
    let mut arr = vec![curr.val];

    while curr.next.is_some() {
        curr = curr.next.unwrap();
        arr.push(curr.val);
    }

    f(&arr)
}

fn f(slice: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if slice.is_empty() {
        None
    } else {
        let pos = slice.len() / 2;
        let mid = slice[pos];
        let left = &slice[..pos];
        let right = &slice[pos + 1..];
        Some(Rc::new(RefCell::new(TreeNode {
            val: mid,
            left: f(left),
            right: f(right),
        })))
    }
}

fn main() {
    let list = Some(Box::new(make_list![
        -26, -19, -14, -8, -5, 3, 7, 11, 15, 21, 24, 32, 35
    ]));
    dbg!(sorted_list_to_bst(list));
}
