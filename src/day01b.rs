use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();

    let mut v: Vec<u64> = stdin.lock().lines().map(|s| s.unwrap().parse::<u64>().unwrap()).into_iter().collect();
    v.sort();

    for s in 0..(v.len() - 2) {
        let mut l: usize = s;
        let mut r: usize = v.len() - 1;
        while l <= r {
            if v[l] + v[r] + v[s] < 2020 {
                l += 1;
            } else if v[l] + v[r] + v[s] > 2020 {
                r -= 1;
            } else {
                println!("{} {} {} {}", v[s], v[l], v[r], v[s] * v[l] * v[r]);
                l += 1;
                r -= 1;
            }
        }
    }
}
