use std::io;
use std::io::prelude::*;
use itertools::Itertools;
use std::iter::once;

fn main() {
    let stdin = io::stdin();

    let mut v: Vec<u64> = stdin.lock().lines().map(|s| s.unwrap().parse::<u64>().unwrap()).collect();
    v.sort();
    v.push(v.last().unwrap() + 3);

    let (r1, _r2, r3) = once(0).chain(v.into_iter()).tuple_windows().map(|(a, b)| b - a).fold((0, 0, 0), |(d1, d2, d3), v| {
        match v {
            1 => (d1 + 1, d2, d3),
            2 => (d1, d2 + 1, d3),
            3 => (d1, d2, d3 + 1),
            _ => panic!("Difference: {}", v)
        }
    });

    println!("{}", r1 * r3);
}
