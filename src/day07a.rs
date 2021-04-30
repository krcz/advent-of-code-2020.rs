use std::io;
use std::io::prelude::*;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

struct BagDag {
    g: HashMap<String, Vec<String>>
}

impl BagDag {
    fn new() -> BagDag {
        BagDag { g: HashMap::new() }
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        //println!("Adding edge {} -> {}", from, to);
        self.g.entry(String::from(from)).or_insert(vec!()).push(String::from(to))
    }

    fn bfs(&self, start: &str) -> HashSet<String> {
        let mut q: VecDeque<&str> = VecDeque::new();
        let mut seen: HashSet<String> = HashSet::new();

        q.push_back(start);
        seen.insert(String::from(start));

        while !q.is_empty() {
            let node = q.pop_front().unwrap();
            for neighbour in self.g.get(&String::from(node)).iter().map(|v| v.iter()).flatten() {
                if !seen.contains(neighbour) {
                    q.push_back(neighbour);
                    seen.insert(String::from(neighbour));
                }
            }
        }

        seen
    }
}

fn main() {
    let stdin = io::stdin();

    let line_re = Regex::new(r#"^(.*) bags contain((?: .* bags?,)* .* bags?\.)$"#).unwrap();
    let bag_re = Regex::new(r#" (\d+) ([^,\.]*) bags?[,\.]"#).unwrap();

    let mut dag = BagDag::new();

    stdin.lock().lines().for_each(|l| {
        let line = l.unwrap();
        let lcaptures = line_re.captures(&line).unwrap();
        let ref name = lcaptures[1];
        //println!("{}", &lcaptures[2]);
        for cap in bag_re.captures_iter(&lcaptures[2]) {
            dag.add_edge(&cap[2], &name);
        }
    });

    let good_bags = dag.bfs("shiny gold");
    println!("{}", good_bags.len() - 1);
}
