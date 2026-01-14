// https://leetcode.com/problems/binary-tree-level-order-traversal

#![allow(unused)]

use std::{cell::RefCell, rc::Rc};

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

fn level_order(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut out = vec![];
    f(root, &mut out, 0);
    out
}

fn f(node: &Option<Rc<RefCell<TreeNode>>>, out: &mut Vec<Vec<i32>>, k: usize) {
    let Some(node) = node else { return };
    if out.len() < k + 1 {
        out.push(vec![]);
    }
    let node_ref = node.borrow();
    out[k].push(node_ref.val);
    f(&node_ref.left, out, k + 1);
    f(&node_ref.right, out, k + 1);
}

fn main() {}
