// https://leetcode.com/problems/flatten-binary-tree-to-linked-list

#![allow(unused)]

use std::{cell::RefCell, rc::Rc};

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

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

fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    let mut curr = root.clone();

    while let Some(c) = curr {
        let mut curr_mut = c.borrow_mut();

        let left = curr_mut.left.take();
        let right = std::mem::replace(&mut curr_mut.right, left);

        drop(curr_mut);

        let mut curr2 = c.clone();
        loop {
            let curr2_ref = curr2.borrow();
            if let Some(right) = curr2_ref.right.clone() {
                drop(curr2_ref);
                curr2 = right;
            } else {
                break;
            }
        }

        let mut curr2_mut = curr2.borrow_mut();

        curr2_mut.right = right;

        drop(curr2_mut);

        let curr_ref = c.borrow();

        curr = curr_ref.right.clone();
    }
}

// fn f(node: &mut Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) {
//     let Some(node) = node else { return };
//     let mut node_mut = node.borrow_mut();
//     let mut prev_right = node_mut.right.take();
//     f(&mut prev_right, None);
//     f(&mut node_mut.left, prev_right);
//     todo!()
// }

fn main() {}
