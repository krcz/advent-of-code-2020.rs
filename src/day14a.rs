use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use regex::Regex;

fn main() {
    let stdin = io::stdin();

    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask: u64 = 0;
    let mut maskv: u64 = 0;

    let re = Regex::new(r#"^(?:mask = ([01X]{36}))|(?:mem\[(\d+)\]) = (\d+)$"#).unwrap();

    for line in stdin.lock().lines() {
        let uline = line.unwrap();
        let captures = re.captures(&uline).unwrap();

        if let Some(masks) = captures.get(1) {
            let (m, mv): (u64, u64) = masks.as_str().chars().fold((0, 0), |(m, mv), c| {
                match c {
                    'X' => (m << 1, mv << 1),
                    '0' => (m << 1 | 1, mv << 1),
                    '1' => (m << 1 | 1, mv << 1 | 1),
                    _ => panic!("Unrecognized character in mask: {}", c)
                }
            });
            mask = m;
            maskv = mv;
        } else {
            let i: u64 = captures[2].parse().unwrap();
            let v: u64 = captures[3].parse().unwrap();

            mem.insert(i, (v & !mask) | (mask & maskv));
        }
    }

    println!("{}", mem.values().sum::<u64>());
}
