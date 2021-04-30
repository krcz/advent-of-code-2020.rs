use std::io;
use std::io::prelude::*;

use itertools::Itertools;

fn condswap(cond: bool, a: i64, b: i64) -> (i64, i64) {
    if cond { (b, a) } else { (a, b) }
}

// finds gcd(a, b) and such (x, y) that x*a + y*b is equal to it, |x| < b, |y| < a
fn _egcd(a: i64, b: i64) -> (i64, (i64, i64)) {
    let swapped = a < b;
    let (a2, b2) = condswap(swapped, a, b);
    if b2 == 0 {
        (a2, condswap(swapped, 1, 0))
    } else if a2 & 1 == 0 {
        if b2 & 1 == 0 {
            let (g, (x, y)) = egcd(a2 >> 1, b2 >> 1);
            // x * a/2 + y * b/2 = g
            // x * a + y * b = 2 * g
            (g << 1, condswap(swapped, x, y))
        } else {
            let (g, (x, y)) = egcd(a2 >> 1, b2);
            // x * a/2 + y * b = g
            if x & 1 == 0 {
                // x even: x/2 * a + y * b = g
                (g, condswap(swapped, x >> 1, y))
            } else if x >= 0 {
                // (x - b) * a/2 + (y + a/2)* b = g
                (g, condswap(swapped, (x - b2) >> 1, y + (a2 >> 1)))
            } else {
                (g, condswap(swapped, (x + b2) >> 1, y - (a2 >> 1)))
            }
        }
    } else {
        let (g, (x, y)) = egcd(a2 - b2, b2);
        (g, condswap(swapped, x, y - x))
    }
}

fn egcd(a: i64, b: i64) -> (i64, (i64, i64)) {
    let (g, (x, y)) = _egcd(a, b);
    println!("{} * {} + {} * {} ?= {}", x, a, y, b, g);
    assert!(x * a + y * b == g, "{} * {} + {} * {} != {}", x, a, y, b, g);
    (g, (x, y))
}

fn inv(a: u64, n: u64) -> u64 {
    let (g, (x, y)) = egcd(a as i64, n as i64);
    // x * a + y * n = 1
    // x * a = 1 + y * n
    assert!(g == 1, "{} and {} not coprime", a, n);
    assert!(n as i64 + x > 0);
    (n as i64 + x) as u64 % n
}

fn main() {
    let stdin = io::stdin();

    let mut lines = stdin.lock().lines();
    let _n: u64 = lines.next().unwrap().unwrap().parse::<u64>().unwrap();
    let bus_riddle: Vec<(u64, u64)> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .enumerate()
        .filter_map(|(i, s)| {
            if s == "x" { None } else {
                let b = s.parse::<u64>().unwrap();
                Some((b, (b - (i as u64 % b))%b))
            }
        })
        .collect();

    let buses: Vec<u64> = bus_riddle.iter().map(|&(b, _)| b).collect();

    for i in 0..buses.len() {
        for j in (i + 1)..buses.len() {
            if egcd(buses[i] as i64, buses[j] as i64).0 != 1 {
                panic!("{} and {} are not co-prime", buses[i], buses[j]);
            }
        }
    };

    let m = buses.iter().fold(1, |a, &b| a * b);

    let solution: u64 = bus_riddle.iter().map(|&(mods, i)| {
        if i == 0 { 0 } else {
            (i * inv((m / mods)%mods, mods))%mods*(m/mods)
        }
    }).sum();

    println!("{}", solution%m);


}
