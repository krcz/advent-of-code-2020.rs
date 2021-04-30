use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();

    //let h: HashMap<char, u32> = vec!(('F', 0), ('B', 1), ('L', 0), ('R', 1)).iter().collect();
    let h: HashMap<char, u32> = vec!(('F', 0), ('B', 1), ('L', 0), ('R', 1)).into_iter().collect();

    let v: Vec<u32> = stdin.lock().lines().map(|s| {
        s.unwrap().chars().fold(0 as u32, |v, c| 2 * v + h.get(&c).unwrap())
    }).collect();

    println!("{}", v.into_iter().fold(0 as u32, |l, r| l.max(r)));
}
