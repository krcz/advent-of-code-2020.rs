use std::io;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashMap;

struct BagDag {
    g: HashMap<String, Vec<(String, u64)>>
}

impl BagDag {
    fn new() -> BagDag {
        BagDag { g: HashMap::new() }
    }

    fn add_edge(&mut self, from: &str, to: &str, weight: u64) {
        //println!("Adding edge {} -> {}", from, to);
        self.g.entry(String::from(from)).or_insert(vec!()).push((String::from(to), weight))
    }

    fn dfs(&self, start: &str) -> u64 {
        self.dfs2(start, &mut HashMap::new())
    }

    fn dfs2(&self, node: &str, cache: &mut HashMap<String, u64>) -> u64 {
        let _n = String::from(node);
        if !cache.contains_key(&_n) {
            let v = self.g.get(&String::from(node))
                    .iter()
                    .map(|v| v.iter())
                    .flatten()
                    .map(|node| node.1 * self.dfs2(&node.0, cache))
                    .sum::<u64>() + 1;
            cache.insert(_n, v);
            v
        } else {
            cache.get(&_n).unwrap().clone()
        }
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
            dag.add_edge(&name, &cap[2], cap[1].parse::<u64>().unwrap());
        }
    });

    println!("{}", dag.dfs("shiny gold") - 1);
}
