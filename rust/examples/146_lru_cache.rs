// https://leetcode.com/problems/lru-cache

#![allow(unused)]

use std::collections::HashMap;

struct LRUCache {
    index: usize,
    buf: Vec<i32>,
    map: HashMap<i32, (i32, usize)>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            index: 0,
            buf: Vec::with_capacity(capacity as usize),
            map: HashMap::with_capacity(capacity as usize),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.map.get_mut(&key) {
            Some((val, index)) => {
                self.buf.remove(*index);
                self.buf.push(key);
                *index = self.buf.len() - 1;
                *val
            }
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        // if self.buf.len() == self.buf.capacity() {
        //     let lru_key = std::mem::replace(&mut self.buf[self.index], key);
        //     self.map.remove(&lru_key);
        //     self.map.insert(key, (value, self.index));
        //     self.index = (self.index + 1) % self.buf.len();
        // } else {
        //     self.map.insert(key, (value, self.buf.len()));
        //     self.buf.push(key);
        // }
        if self.buf.len() == self.buf.capacity() {}
    }
}

fn check(commands: &[&str], vals: &[Vec<i32>], expected: &[Option<i32>]) {
    commands.iter().zip(vals).zip(expected).fold(
        None::<LRUCache>,
        |mut cache, ((ix, vals), expected)| {
            match (*ix, vals.as_slice()) {
                ("LRUCache", [capacity]) => {
                    cache = Some(LRUCache::new(*capacity));
                    assert_eq!(None, *expected);
                }
                ("put", [key, value]) => {
                    cache.as_mut().unwrap().put(*key, *value);
                    assert_eq!(None, *expected);
                }
                ("get", [key]) => {
                    let val = cache.as_mut().unwrap().get(*key);
                    assert_eq!(Some(val), *expected);
                }
                _ => panic!("invalid instruction or values"),
            }
            cache
        },
    );
}

fn main() {
    check(
        &[
            "LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get",
        ],
        &[
            vec![2],
            vec![1, 1],
            vec![2, 2],
            vec![1],
            vec![3, 3],
            vec![2],
            vec![4, 4],
            vec![1],
            vec![3],
            vec![4],
        ],
        &[
            None,
            None,
            None,
            Some(1),
            None,
            Some(-1),
            None,
            Some(-1),
            Some(3),
            Some(4),
        ],
    );
}
