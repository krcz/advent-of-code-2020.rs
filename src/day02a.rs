use std::io;
use std::io::prelude::*;
use regex::Regex;

fn main() {
    let stdin = io::stdin();

    let re = Regex::new(r"^(\d+)-(\d+)\s+(\w):\s+(\w+)$").unwrap();

    let cnt: u32 = stdin.lock().lines().map(|line| {
        let uline = line.unwrap();
        let cap = re.captures(&uline).unwrap();
        let min = cap[1].parse::<u32>().unwrap();
        let max = cap[2].parse::<u32>().unwrap();
        let c = cap[3].chars().next().unwrap();
        let ref s = cap[4];
        let cnt = s.chars().filter(|cc| cc == &c).count() as u32;
        if min <= cnt && cnt <= max { 1 } else { 0 }
    }).sum();

    println!("{}", cnt);
}
