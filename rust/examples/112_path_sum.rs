// https://leetcode.com/problems/path-sum/

#![allow(clippy::bool_assert_comparison)]

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

fn has_path_sum(root: &Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    root.as_ref().map(|x| f(x, target_sum)).unwrap_or(false)
}

fn f(node: &Rc<RefCell<TreeNode>>, target_sum: i32) -> bool {
    let node_ref = node.borrow();
    let next_target_sum = target_sum - node_ref.val;
    if next_target_sum == 0 && node_ref.left.is_none() && node_ref.right.is_none() {
        true
    } else {
        node_ref
            .left
            .as_ref()
            .map(|x| f(x, next_target_sum))
            .unwrap_or(false)
            || node_ref
                .right
                .as_ref()
                .map(|x| f(x, next_target_sum))
                .unwrap_or(false)
    }
}

fn main() {
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
        right: None,
    })));
    assert_eq!(has_path_sum(&tree, 1), false);
}
