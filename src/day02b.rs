use std::io;
use std::io::prelude::*;
use regex::Regex;

fn main() {
    let stdin = io::stdin();

    let re = Regex::new(r"^(\d+)-(\d+)\s+(\w):\s+(\w+)$").unwrap();

    let cnt: u32 = stdin.lock().lines().map(|line| {
        let uline = line.unwrap();
        let cap = re.captures(&uline).unwrap();
        let p1 = cap[1].parse::<usize>().unwrap();
        let p2 = cap[2].parse::<usize>().unwrap();
        let c = cap[3].chars().next().unwrap();
        let ref s = cap[4];
        if (s.chars().nth(p1 - 1).unwrap() == c) ^ (s.chars().nth(p2 - 1).unwrap() == c) { 1 } else { 0 }
    }).sum();

    println!("{}", cnt);
}
