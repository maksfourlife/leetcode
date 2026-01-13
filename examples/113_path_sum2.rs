// https://leetcode.com/problems/path-sum/

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

fn has_path_sum(root: &Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
    root.as_ref()
        .map(|x| f(x, target_sum, vec![]))
        .unwrap_or_default()
}

fn f(node: &Rc<RefCell<TreeNode>>, target_sum: i32, mut path: Vec<i32>) -> Vec<Vec<i32>> {
    let node_ref = node.borrow();
    path.push(node_ref.val);
    let next_target_sum = target_sum - node_ref.val;
    if next_target_sum == 0 && node_ref.left.is_none() && node_ref.right.is_none() {
        vec![path]
    } else {
        let mut out = vec![];
        for branch in [&node_ref.left, &node_ref.right] {
            if let Some(t) = branch.as_ref().map(|x| f(x, next_target_sum, path.clone())) {
                out.extend(t)
            }
        }
        out
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
    dbg!(has_path_sum(&tree, 1));
}
