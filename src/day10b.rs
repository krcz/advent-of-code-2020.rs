use std::io;
use std::io::prelude::*;
use itertools::Itertools;
use std::iter::once;

fn main() {
    let stdin = io::stdin();

    let mut v: Vec<u64> = stdin.lock().lines().map(|s| s.unwrap().parse::<u64>().unwrap()).collect();
    v.sort();
    v.push(v.last().unwrap() + 3);

    let (r0, _r1, _r2) = once(0).chain(v.into_iter()).tuple_windows().map(|(a, b)| b - a).fold((1 as u64, 0 as u64, 0 as u64), |(w0, w1, w2), v| {
        match v {
            1 => (w0 + w1 + w2, w0, w1),
            2 => (w0 + w1, 0, w0),
            3 => (w0, 0, 0),
            _ => panic!("Difference: {}", v)
        }
    });

    println!("{}", r0);
}
