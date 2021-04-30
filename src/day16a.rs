use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

use itertools::Itertools;
use regex::Regex;

fn parse_ticket(s: &str) -> Vec<u64> {
    s.split(",").map(|n| n.parse().unwrap()).collect()
}

fn merge_ranges(ranges: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut sranges = ranges.clone();
    sranges.sort();

    let mut res: Vec<(u64, u64)> = vec!(*sranges.first().unwrap());

    for &(l, r) in sranges.iter().dropping(1) {
        let (_, lr) = res.last_mut().unwrap();
        if l > *lr + 1 {
            res.push((l, r))
        } else {
            *lr = r
        }
    }

    res
}

fn in_ranges(el: u64, ranges: &Vec<(u64, u64)>) -> bool {
    ranges.iter().any(|&(rl, rr)| { rl <= el && el <= rr })
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| String::from(l.unwrap().trim()));

    let rule_re = Regex::new(r#"(\w+): (\d+-\d+(?: or \d+-\d+)*)"#).unwrap();
    let range_re = Regex::new(r#"(\d+)-(\d+)"#).unwrap();

    let rules: HashMap<String, Vec<(u64, u64)>> = (&mut lines).take_while(|l| !l.is_empty()).map(|l| {
        let captures = rule_re.captures(&l).unwrap();
        let ranges: Vec<(u64, u64)> = captures[2].split(" or ").map(|s| {
            let rcaptures = range_re.captures(s).unwrap();
            (rcaptures[1].parse().unwrap(), rcaptures[2].parse().unwrap())
        }).collect();
        (String::from(&captures[1]), ranges)
    }).collect();

    let my_ticket = parse_ticket(&(&mut lines).dropping(1).next().unwrap());

    let other_tickets: Vec<_> = (&mut lines).dropping(2).map(|l| parse_ticket(&l)).collect();

    let ranges_sum = merge_ranges(&rules.values().flatten().copied().collect());

    let errors = other_tickets.iter().flatten().filter(|&&el| !in_ranges(el, &ranges_sum));

    println!("{}", errors.sum::<u64>());
}
