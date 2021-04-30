use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use itertools::Itertools;

fn main() {
    let stdin = io::stdin();

    //let h: HashMap<char, u32> = vec!(('F', 0), ('B', 1), ('L', 0), ('R', 1)).iter().collect();
    let h: HashMap<char, u32> = vec!(('F', 0), ('B', 1), ('L', 0), ('R', 1)).into_iter().collect();

    let mut v: Vec<u32> = stdin.lock().lines().map(|s| {
        s.unwrap().chars().fold(0 as u32, |v, c| 2 * v + h.get(&c).unwrap())
    }).collect();

    v.sort();

    let result = v.into_iter().tuple_windows().filter_map(|(l, r)| if r - l > 1 { Some(l + 1) } else { None }).next().unwrap();

    println!("{}", result);
}
