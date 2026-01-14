// https://leetcode.com/problems/validate-binary-search-tree

#![allow(clippy::bool_assert_comparison)]

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

use std::cmp::Ordering;

fn is_valid_bst(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    f(root, &mut Vec::with_capacity(200), 0)
}

fn f(node: &Option<Rc<RefCell<TreeNode>>>, steps: &mut Vec<(i32, Ordering)>, k: usize) -> bool {
    match node.as_ref().map(|x| x.borrow()) {
        Some(node) => {
            (0..k).all(|i| {
                let step = &steps[i];
                node.val.cmp(&step.0) == step.1
            }) && {
                if steps.len() < k + 1 {
                    steps.push((node.val, Ordering::Less));
                } else {
                    steps[k] = (node.val, Ordering::Less);
                }
                f(&node.left, steps, k + 1)
            } && {
                steps[k].1 = Ordering::Greater;
                f(&node.right, steps, k + 1)
            }
        }
        None => true,
    }
}

fn main() {
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    })));
    assert_eq!(is_valid_bst(&tree), false);
}
