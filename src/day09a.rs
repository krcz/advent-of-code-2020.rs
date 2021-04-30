#![feature(map_entry_replace)]
#![feature(option_result_contains)]

use std::collections::hash_map::Entry;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::VecDeque;

use itertools::{FoldWhile, Itertools};

struct WakeBuf {
    size: usize,
    wake: VecDeque<u64>,
    cnt: HashMap<u64, u8>
}

impl WakeBuf {
    fn new(size: usize) -> WakeBuf {
        WakeBuf { size, wake: VecDeque::with_capacity(size), cnt: HashMap::new() }
    }

    fn push(&mut self, v: u64) {
        if self.wake.len() == self.size {
            let oldest = self.wake.pop_front().unwrap();
            let entry = self.cnt.entry(oldest);
            match entry {
                Entry::Occupied(e) => {
                    let vv = *e.get();
                    if vv == 1 { e.remove(); } else { e.replace_entry(vv - 1);}
                },
                Entry::Vacant(_) => {}
            }
        }
        self.wake.push_back(v);
        self.cnt.entry(v).and_modify(|c| *c += 1).or_insert(1);
    }

    fn is_ok(&self, v: u64) -> bool {
        self.wake.len() < self.size || self.wake
            .iter()
            .any(|el| {
                if el + el == v {
                    self.cnt.get(el).map_or(false, |c| *c >= 2)
                } else {
                    v > *el && self.cnt.contains_key(&(v - el))
                }
            })
    }
}

fn main() {
    let stdin = io::stdin();
    let mut wb: WakeBuf = WakeBuf::new(25);

    let result = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<u64>().unwrap())
        .filter(|k| {
            let res = !wb.is_ok(*k);
            wb.push(*k);
            res
        }).next();

    println!("{}", result.unwrap());
}
