// https://leetcode.com/problems/unique-binary-search-trees
// Given an integer n, return the number of structurally unique BST's (binary search trees) which has exactly n nodes of unique values from 1 to n.

#![allow(clippy::needless_bool, clippy::useless_vec)]

use std::{fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Tree<T> {
    val: T,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

// #[derive(PartialEq, Eq, PartialOrd, Ord)]
// struct TreeCompare {
//     left: Option<Box<Self>>,
//     right: Option<Box<Self>>,
// }

impl<T> fmt::Display for Tree<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.left, &self.right) {
            (Some(left), Some(right)) => write!(f, "({left} {} {right})", &self.val),
            (Some(left), None) => write!(f, "({left} {})", &self.val),
            (None, Some(right)) => write!(f, "({} {right})", &self.val),
            (None, None) => write!(f, "({})", &self.val),
        }
    }
}

impl<T> Tree<T> {
    // fn compare(&self) -> TreeCompare {
    //     TreeCompare {
    //         left: self.left.as_ref().map(|x| Box::new(x.compare())),
    //         right: self.right.as_ref().map(|x| Box::new(x.compare())),
    //     }
    // }

    fn empty(self) -> Tree<()> {
        Tree {
            val: (),
            left: self.left.map(|x| Box::new(x.empty())),
            right: self.right.map(|x| Box::new(x.empty())),
        }
    }

    fn insert(&mut self, cursor: &TreeCursor, val: T) {
        let subtree = self.get_mut(cursor);
        subtree.right = Some(Box::new(Tree {
            val,
            left: subtree.right.take(),
            right: None,
        }));
    }

    fn get_mut(&mut self, cursor: &TreeCursor) -> &mut Self {
        let mut curr = self;
        for dir in &cursor.dirs {
            curr = match dir {
                Dir::Left => curr.left.as_mut().unwrap(),
                Dir::Right => curr.right.as_mut().unwrap(),
            };
        }
        curr
    }

    fn next(&self, cursor: &mut TreeCursor) {
        let mut ptrs = vec![self];

        for dir in &cursor.dirs {
            let curr = ptrs.last().unwrap();
            ptrs.push(match dir {
                Dir::Left => curr.left.as_ref().unwrap(),
                Dir::Right => curr.right.as_ref().unwrap(),
            });
        }

        let curr = ptrs.last().unwrap();

        if curr.left.is_some() {
            cursor.dirs.push(Dir::Left);
        } else if curr.right.is_some() {
            cursor.dirs.push(Dir::Right);
        } else {
            // try to find the next right node and set it as right
            while !cursor.dirs.is_empty() {
                let curr = ptrs[cursor.dirs.len() - 1];
                if cursor
                    .dirs
                    .pop_if(|dir| !(*dir == Dir::Left && curr.right.is_some()))
                    .is_none()
                {
                    break;
                }
            }
            if let Some(last) = cursor.dirs.last_mut() {
                *last = Dir::Right;
            } else {
                cursor.exhausted = true;
            }
        }
    }
}

#[derive(Debug, Default)]
struct TreeCursor {
    dirs: Vec<Dir>,
    exhausted: bool,
}

impl TreeCursor {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Left,
    Right,
}

fn f(n: usize) -> usize {
    let mut prev = vec![Tree {
        val: 1,
        left: None,
        right: None,
    }];
    for x in 2..=n {
        let mut curr: Vec<Tree<_>> = vec![];
        prev.iter().for_each(|tree| {
            curr.push(Tree {
                val: x,
                left: Some(Box::new(tree.clone())),
                right: None,
            });
            let mut cursor = TreeCursor::new();
            while !cursor.exhausted {
                let mut tree2 = tree.clone();
                tree2.insert(&cursor, x);
                curr.push(tree2);
                tree.next(&mut cursor);
            }
        });
        dbg!(&curr);
        // for x in &curr {
        //     println!("{x}");
        // }
        // println!("---");
        curr.sort_unstable_by_key(|x| x.clone().empty());
        curr.dedup_by_key(|x| x.clone().empty());
        prev = curr;
    }
    prev.len()
}

fn num_trees(n: i32) -> i32 {
    f(n as usize) as i32
}

fn main() {
    dbg!(f(4));
    // let tree = Tree {
    //     val: 2,
    //     left: Some(Box::new(Tree {
    //         val: 1,
    //         left: None,
    //         right: None,
    //     })),
    //     right: Some(Box::new(Tree {
    //         val: 4,
    //         left: Some(Box::new(Tree {
    //             val: 3,
    //             left: None,
    //             right: None,
    //         })),
    //         right: Some(Box::new(Tree {
    //             val: 5,
    //             left: None,
    //             right: None,
    //         })),
    //     })),
    // };
    // dbg!(&tree);
    // let tree = Tree {
    //     val: 6,
    //     left: Some(Box::new(Tree {
    //         val: 2,
    //         left: Some(Box::new(Tree {
    //             val: 1,
    //             left: None,
    //             right: None,
    //         })),
    //         right: Some(Box::new(Tree {
    //             val: 4,
    //             left: Some(Box::new(Tree {
    //                 val: 3,
    //                 left: None,
    //                 right: None,
    //             })),
    //             right: Some(Box::new(Tree {
    //                 val: 5,
    //                 left: None,
    //                 right: None,
    //             })),
    //         })),
    //     })),
    //     right: None,
    // };
    // let tree = Tree {
    //     val: 1,
    //     left: None,
    //     right: Some(Box::new(Tree {
    //         val: 2,
    //         left: None,
    //         right: None,
    //     })),
    // };
    // println!("{tree}");
    // let mut cursor = TreeCursor::new();
    // while !cursor.exhausted {
    //     tree.next(&mut cursor);
    //     let mut tree2 = tree.clone();
    //     tree2.insert(&cursor, 7);
    //     println!("{tree2}");
    // }
}
