// https://leetcode.com/problems/unique-binary-search-trees
// Given an integer n, return the number of structurally unique BST's (binary search trees) which has exactly n nodes of unique values from 1 to n.

#![allow(clippy::needless_bool, clippy::useless_vec)]

use std::{cell::RefCell, fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Tree<T> {
    val: T,
    left: Option<Rc<RefCell<Self>>>,
    right: Option<Rc<RefCell<Self>>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct TreeCompare {
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

impl<T> fmt::Display for Tree<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.left, &self.right) {
            (Some(left), Some(right)) => {
                write!(f, "({} {} {})", left.borrow(), right.borrow(), &self.val)
            }
            (Some(left), None) => write!(f, "({} {})", left.borrow(), &self.val),
            (None, Some(right)) => write!(f, "({} {})", &self.val, right.borrow()),
            (None, None) => write!(f, "({})", &self.val),
        }
    }
}

impl<T> Tree<T> {
    fn compare(&self) -> TreeCompare {
        TreeCompare {
            left: self.left.as_ref().map(|x| Box::new(x.borrow().compare())),
            right: self.right.as_ref().map(|x| Box::new(x.borrow().compare())),
        }
    }

    fn clone_deep(&self) -> Self
    where
        T: Clone,
    {
        let f = |x: &Option<Rc<RefCell<Tree<T>>>>| {
            x.as_ref()
                .map(|x| Rc::new(RefCell::new(x.borrow().clone_deep())))
        };
        Self {
            val: self.val.clone(),
            left: f(&self.left),
            right: f(&self.right),
        }
    }
}

fn tree_insert<T>(tree: Rc<RefCell<Tree<T>>>, cursor: &TreeCursor, val: T)
where
    T: Clone,
{
    let subtree = tree_get(tree, cursor);
    let mut subtree_mut = subtree.borrow_mut();
    subtree_mut.right = Some(Rc::new(RefCell::new(Tree {
        val,
        left: subtree_mut.right.take(),
        right: None,
    })));

    // let tree2 = Rc::new(RefCell::new(Tree {
    //     val: tree.borrow().val.clone(),
    //     left: None,
    //     right: None,
    // }));

    // let mut curr = tree;
    // let mut curr2 = tree2;

    // for dir in &cursor.dirs {
    //     match dir {
    //         Dir::Left => {
    //             let curr_left = curr.borrow().left.clone().unwrap();
    //             curr = curr_left;

    //             let curr_ref = curr.borrow();
    //             let mut curr2_ref = curr2.borrow_mut();

    //             let left2 = Rc::new(RefCell::new(Tree {
    //                 val: curr_ref.val.clone(),
    //                 left: None,
    //                 right: None,
    //             }));
    //             curr2_ref.left = Some(left2.clone());
    //             curr2_ref.right = curr_ref.right.clone();

    //             drop(curr2_ref);

    //             curr2 = left2;
    //         }
    //         Dir::Right => {
    //             todo!()
    //         }
    //     };
    // }
}

fn tree_get<T>(tree: Rc<RefCell<Tree<T>>>, cursor: &TreeCursor) -> Rc<RefCell<Tree<T>>> {
    let mut curr = tree;
    for dir in &cursor.dirs {
        curr = match dir {
            Dir::Left => curr.borrow().left.clone().unwrap(),
            Dir::Right => curr.borrow().right.clone().unwrap(),
        };
    }
    curr
}

fn tree_next<T>(tree: Rc<RefCell<Tree<T>>>, cursor: &mut TreeCursor) {
    let mut ptrs = vec![tree];

    for dir in &cursor.dirs {
        let curr = ptrs.last().unwrap().borrow();
        let next = match dir {
            Dir::Left => curr.left.as_ref().unwrap(),
            Dir::Right => curr.right.as_ref().unwrap(),
        }
        .clone();
        drop(curr);
        ptrs.push(next);
    }

    let curr = ptrs.last().unwrap().borrow();

    if curr.left.is_some() {
        cursor.dirs.push(Dir::Left);
    } else if curr.right.is_some() {
        cursor.dirs.push(Dir::Right);
    } else {
        // try to find the next right node and set it as right
        while !cursor.dirs.is_empty() {
            let curr = &ptrs[cursor.dirs.len() - 1].borrow();
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

fn f(n: usize) -> Vec<Rc<RefCell<Tree<usize>>>> {
    let mut prev = vec![Rc::new(RefCell::new(Tree {
        val: 1,
        left: None,
        right: None,
    }))];
    for x in 2..=n {
        let mut curr: Vec<Rc<RefCell<Tree<_>>>> = vec![];
        prev.iter().for_each(|tree| {
            curr.push(Rc::new(RefCell::new(Tree {
                val: x,
                left: Some(tree.clone()),
                right: None,
            })));
            let mut cursor = TreeCursor::new();
            while !cursor.exhausted {
                let tree2 = Rc::new(RefCell::new(tree.borrow().clone_deep()));
                tree_insert(tree2.clone(), &cursor, x);
                curr.push(tree2);
                tree_next(tree.clone(), &mut cursor);
            }
        });
        // dbg!(&curr);
        // for x in &curr {
        //     println!("{x}");
        // }
        // println!("---");
        curr.sort_unstable_by_key(|x| x.borrow().compare());
        curr.dedup_by_key(|x| x.borrow().compare());
        prev = curr;
    }
    prev
}

// fn num_trees(n: i32) -> i32 {
//     f(n as usize) as i32
// }

fn main() {
    dbg!(f(2));
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
