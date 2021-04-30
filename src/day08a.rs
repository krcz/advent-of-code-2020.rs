#[macro_use]
extern crate lazy_static;

use std::io;
use std::io::prelude::*;

use regex::Regex;
use fixedbitset::FixedBitSet;

enum Instruction{
    Acc(i64), Jmp(i64), Nop
}

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r#"^(acc|jmp|nop) ([-+]\d+)$"#).unwrap();
}

impl Instruction {
    fn from(s: &str) -> Instruction {
        let captures = LINE_RE.captures(s).unwrap();
        match &captures[1] {
            "nop" => Instruction::Nop,
            "acc" => Instruction::Acc(captures[2].parse::<i64>().unwrap()),
            "jmp" => Instruction::Jmp(captures[2].parse::<i64>().unwrap()),
            _ => panic!("Unknown instruction type")
        }
    }
}

fn main() {
    let stdin = io::stdin();

    let code: Vec<Instruction> = stdin.lock().lines().map(|s| {
        Instruction::from(&s.unwrap())
    }).collect();

    let mut bs = FixedBitSet::with_capacity(code.len());

    let mut acc: i64 = 0;
    let mut i: usize = 0;

    while !bs[i] {
        bs.put(i);
        match code[i] {
            Instruction::Acc(v) => { acc += v; i += 1 },
            Instruction::Jmp(v) => {i = (i as i64 + v) as usize },
            Instruction::Nop => {i += 1}
        }
    }

    println!("{}", acc);
}
