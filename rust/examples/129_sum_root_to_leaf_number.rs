// https://leetcode.com/problems/sum-root-to-leaf-numbers/

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

fn sum_numbers(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(x) => f(x, 0),
        None => 0,
    }
}

fn f(node: &Rc<RefCell<TreeNode>>, s: i32) -> i32 {
    let node_ref = node.borrow();
    let s = s * 10 + node_ref.val;
    if node_ref.left.is_none() && node_ref.right.is_none() {
        s
    } else {
        [&node_ref.left, &node_ref.right]
            .into_iter()
            .flat_map(|x| x.as_ref().map(|x| f(x, s)))
            .sum()
    }
}

fn main() {}
