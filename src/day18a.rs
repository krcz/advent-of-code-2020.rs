use std::io;
use std::io::prelude::*;

use regex::Regex;

fn eval1<'a>(tokens: &'a[&'a str]) -> (&'a[&'a str], i64) {
    if tokens[0] == "(" {
        let (rest, v) = eval2(&tokens[1..]);
        assert!(rest[0] == ")");
        (&rest[1..], v)
    } else {
        (&tokens[1..], tokens[0].parse().unwrap())
    }
}

fn eval2<'a>(tokens: &'a[&'a str]) -> (&'a [&'a str], i64) {
    let (mut rest, mut v) = eval1(tokens);

    while !rest.is_empty() && (rest[0] == "+" || rest[0] == "*") {
        let (rest2, v2) = eval1(&rest[1..]);
        match rest[0] {
            "+" => v += v2,
            "*" => v *= v2,
            _ => panic!("Impossible")
        };
        rest = rest2;
    }

    (&rest, v)
}

fn eval(expr: &str) -> i64 {
    let tokens: Vec<&str> = Regex::new(r#"\d+|[\(\)*+]"#).unwrap().find_iter(expr.trim()).map(|m| m.as_str().trim()).collect();
    println!("{:?}", tokens);
    eval2(tokens.as_slice()).1
}

fn main() {
    let stdin = io::stdin();

    let results: Vec<i64> = stdin.lock().lines().map(|s| eval(&s.unwrap())).collect();

    println!("{:?}", results.iter().sum::<i64>());
}
