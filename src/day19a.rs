use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;

use regex::Regex;

#[derive(Debug)]
enum Elem<'b> {
    RuleRef(usize),
    Str(&'b str)
}

struct Ruleset<'b> {
    rules: Vec<Vec<Vec<Elem<'b>>>>
}

impl<'b> Ruleset<'b> {
    fn matches(&self, s: &str) -> bool {
        let mut _mp: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
        self.mp(0, 0, s, &mut _mp).contains(&s.len())
    }

    fn mp<'a>(&self, r: usize, left: usize, s: &str, _mp: &'a mut HashMap<(usize, usize), Vec<usize>>) -> &'a Vec<usize> {
        if !_mp.contains_key(&(r, left)) {
            let mut all_ends: Vec<usize> = self.rules[r].iter().flat_map(|variant| {
                let mut ends: Vec<usize> = vec!(left);

                for el in variant.iter() {
                    match el {
                        Elem::Str(pat) => {
                            ends = ends.iter().filter_map(|&end| {
                                if s[end..].starts_with(pat) {
                                    Some(end + pat.len())
                                } else {
                                    None
                                }
                            }).collect();
                        },
                        Elem::RuleRef(r2) => {
                            ends = ends.iter().flat_map(|end| {
                                self.mp(*r2, *end, s, _mp).clone()
                            }).collect();
                        }
                    }
                }
                ends
            }).collect();
            all_ends.sort();
            all_ends.dedup();

            //println!("rule {} ({:?}) pos {}: {:?}", r, self.rules[r], left, all_ends);
            _mp.insert((r, left), all_ends);
        }
        _mp.get(&(r, left)).unwrap()
    }
}

impl<'b> FromIterator<Vec<Vec<Elem<'b>>>> for Ruleset<'b> {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = Vec<Vec<Elem<'b>>>> {
        Ruleset { rules: iter.into_iter().collect() }
    }
}

fn parse_rule_lines<'b>(lines: &'b Vec<String>) -> Ruleset<'b> {
    let rule_re = Regex::new(r#"^(\d+):\s*([^\s].*)"#).unwrap();

    let mut unsorted_rules: Vec<(usize, Vec<Vec<Elem<'b>>>)> = lines.iter().enumerate().map(|(i, l)| {
        let captures = rule_re.captures(&l).unwrap();
        let variants: Vec<Vec<Elem<'b>>> = captures.get(2).unwrap().as_str().split('|').map(|vars| {
            vars.trim().split(' ').map(|el| {
                let elt = el.trim();
                if elt.starts_with("\"") {
                    Elem::Str(&elt[1..(elt.len() - 1)])
                } else {
                    Elem::RuleRef(elt.parse().unwrap())
                }
            }).collect()
        }).collect();
        (captures[1].parse::<usize>().unwrap(), variants)
    }).collect();

    unsorted_rules.sort_by_key(|(i, _)| *i);
    unsorted_rules.into_iter().map(|(_, v)| v).collect()
}

fn main() {
    let stdin = io::stdin();
    let mut lines_it = stdin.lock().lines();

    let rule_lines: Vec<String> = lines_it.by_ref().map(|l| String::from(l.unwrap())).take_while(|l| !l.is_empty()).collect();

    let rules: Ruleset = parse_rule_lines(&rule_lines);

    let valid: Vec<_> = lines_it.filter_map(|l| {
        let ll = l.unwrap();
        //println!("{}", ll);
        if rules.matches(&ll) {
            Some(ll)
        } else {
            None
        }
    }).collect();

    valid.iter().for_each(|s| println!("{}", s));
    println!("{}", valid.len());
}
