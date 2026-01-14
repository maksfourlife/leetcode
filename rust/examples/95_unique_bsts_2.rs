// https://leetcode.com/problems/unique-binary-search-trees-ii

#![allow(clippy::useless_vec, clippy::let_and_return)]

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

impl TreeNode {
    fn add(&mut self, x: i32) {
        self.val += x;
        if let Some(t) = &self.left {
            t.borrow_mut().add(x);
        }
        if let Some(t) = &self.right {
            t.borrow_mut().add(x);
        }
    }
}

fn deep_clone_tree(tree: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    tree.as_ref().map(|x| {
        let r = x.borrow();
        Rc::new(RefCell::new(TreeNode {
            val: r.val,
            left: deep_clone_tree(&r.left),
            right: deep_clone_tree(&r.right),
        }))
    })
}

fn generate_trees(n: usize) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let mut trees = vec![vec![]; n + 1];
    trees[0] = vec![None];
    trees[1] = vec![Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    })))];
    (2..n + 1).for_each(|k| {
        (1..=k).for_each(|x| {
            let x: Vec<_> = trees[x - 1]
                .iter()
                .flat_map(|left| {
                    trees[k - x].iter().map(move |right| {
                        let mut right = deep_clone_tree(right);
                        if let Some(t) = &mut right {
                            t.borrow_mut().add(x as i32);
                        }
                        let res = Some(Rc::new(RefCell::new(TreeNode {
                            val: x as i32,
                            left: left.clone(),
                            right,
                        })));
                        res
                    })
                })
                .collect();
            trees[k].extend(x);
        });
    });
    trees.swap_remove(n)
}

fn main() {
    generate_trees(8);
}
