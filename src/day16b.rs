use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
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

fn max_match_dfs(
    name: &String,
    graph: &HashMap<String, Vec<usize>>,
    ass: &mut HashMap<String, usize>,
    rass: &mut HashMap<usize, String>,
    seen: &mut HashSet<String>
) -> bool {
    for &pos in graph[name].iter() {
        if !rass.contains_key(&pos) {
            ass.insert(name.clone(), pos);
            rass.insert(pos, name.clone());
            return true;
        }
    }

    for &pos in graph[name].iter() {
        let name2_opt: Option<String> = rass.get(&pos).cloned();
        if let Some(name2) = name2_opt {
            if !seen.contains(&name2) {
                seen.insert(name2.clone());
                if max_match_dfs(&name2, graph, ass, rass, seen) {
                    ass.insert(name.clone(), pos);
                    rass.insert(pos, name.clone());
                    return true;
                }
            }
        }
    }
    return false;

}

fn max_match(graph: &HashMap<String, Vec<usize>>) -> HashMap<String, usize> {
    let mut ass: HashMap<String, usize> = HashMap::new();
    let mut rass: HashMap<usize, String> = HashMap::new();

    loop {
        let mut seen: HashSet<String> = HashSet::new();
        if !graph.keys().any(|name| {
            !ass.contains_key(name) && {
                seen.insert(name.clone());
                max_match_dfs(name, graph, &mut ass, &mut rass, &mut seen)
            }
        }) {
            break;
        }
    }

    ass
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| String::from(l.unwrap().trim()));

    let rule_re = Regex::new(r#"(.+): (\d+-\d+(?: or \d+-\d+)*)"#).unwrap();
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

    let good_tickets: Vec<&Vec<u64>> = other_tickets.iter().filter(|&ticket| ticket.iter().all(|&el| in_ranges(el, &ranges_sum))).collect();

    let n = my_ticket.len();

    let pos_numbers: Vec<Vec<u64>> = (0..n).map(|i| {
        good_tickets.iter().map(|ticket| ticket[i]).collect()
    }).collect();



    let match_graph: HashMap<String, Vec<usize>> = rules.iter().map(|(name, ranges)| {
        let sranges = merge_ranges(ranges);
        let good_pos: Vec<usize> = pos_numbers.iter().enumerate().filter_map(|(i, numbers)| {
            if numbers.iter().all(|&el| in_ranges(el, &sranges)) {
                Some(i)
            } else {
                None
            }
        }).collect();

        (name.clone(), good_pos)
    }).collect();

    let mmatch = max_match(&match_graph);

    println!("{} {:?}", mmatch.len(), mmatch);

    let res: u64 = mmatch.iter().filter(|(k, v)| k.starts_with("departure")).fold(1, |acc, (_, &v)| acc * my_ticket[v]);

    println!("{}", res);

}
