#[macro_use]
extern crate lazy_static;

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

use regex::Regex;
use fixedbitset::FixedBitSet;

enum Instruction{
    Acc(i64), Jmp(i64), Nop(i64)
}

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r#"^(acc|jmp|nop) ([-+]\d+)$"#).unwrap();
}

impl Instruction {
    fn from(s: &str) -> Instruction {
        let captures = LINE_RE.captures(s).unwrap();
        let v: i64 = captures[2].parse::<i64>().unwrap();
        match &captures[1] {
            "nop" => Instruction::Nop(v),
            "acc" => Instruction::Acc(v),
            "jmp" => Instruction::Jmp(v),
            _ => panic!("Unknown instruction type")
        }
    }
}

fn code_finality(code: &Vec<Instruction>) -> FixedBitSet {
    let mut backlinks: Vec<Vec<usize>> = vec![vec!(); code.len() + 1];

    for (i, ins) in code.iter().enumerate() {
        match ins {
            Instruction::Jmp(v) => { backlinks[(i as i64 + v) as usize].push(i) },
            _ => { backlinks[i + 1].push(i) }
        }
    }

    let mut bs = FixedBitSet::with_capacity(code.len() + 1);
    let mut q: VecDeque<usize> = VecDeque::new();
    let n = code.len();

    bs.put(n);
    q.push_back(n);

    while let Some(i) = q.pop_front() {
        for j in &backlinks[i] {
            if !bs.contains(*j) {
                bs.put(*j);
                q.push_back(*j);
            }
        }
    }

    bs
}

fn main() {
    let stdin = io::stdin();

    let code: Vec<Instruction> = stdin.lock().lines().map(|s| {
        Instruction::from(&s.unwrap())
    }).collect();

    let cf = code_finality(&code);

    let mut fixed = false;

    let mut acc: i64 = 0;
    let mut i: usize = 0;
    let n = code.len();

    while i < n {
        match code[i] {
            Instruction::Acc(v) => { acc += v; i += 1 },
            Instruction::Jmp(v) => {
                if !fixed && cf.contains(i + 1) {
                    fixed = true;
                    i += 1;
                } else {
                    i = (i as i64 + v) as usize;
                }
            }
            Instruction::Nop(v) => {
                let t = i as i64 + v;
                if !fixed && t >= 0 && t <= n as i64 && cf.contains(t as usize) {
                    fixed = true;
                    i = t as usize;
                } else {
                    i += 1;
                }
            }
        }
    }

    println!("{}", acc);
}
