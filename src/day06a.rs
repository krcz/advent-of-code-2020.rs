use std::io;
use std::io::prelude::*;

use itertools::Itertools;

fn bitify(s: &str) -> u64 {
    s.chars().fold(0, |bits, c| bits | (1 << (c as u32 - 'a' as u32)))
}

fn main() {
    let stdin = io::stdin();

    let result: u32 = stdin.lock().lines().map(|l| l.unwrap()).batching(|it| {
        let mut it2 = it.take_while(|l| !l.is_empty());
        it2.next().map(|v| it2.fold(bitify(&v), |bits, l| { bits & bitify(&l) })).map(|bits| bits.count_ones())
    }).sum();

    println!("{}", result);
}
