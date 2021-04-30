use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();

    let mut lines = stdin.lock().lines();
    let n: u64 = lines.next().unwrap().unwrap().parse::<u64>().unwrap();
    let buses: Vec<u64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .filter_map(|s| if s == "x" { None } else {Some(s.parse::<u64>().unwrap())})
        .collect();

    let first_bus = buses.iter().min_by_key(|&b| (b - n % b) % b).unwrap();

    println!("{}", first_bus * ((first_bus - n % first_bus) % first_bus));

}
