use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::iter::Iterator;
use regex::Regex;

fn date_between(low: u32, high: u32) -> Box<dyn Fn(&&str) -> bool> {
    Box::new(move |s| {
        s.len() == 4 && {
            let date: Option<u32> = s.parse().ok();
            date.map_or(false, |v| low <= v && v <= high)
        }
    })
}

fn height() -> Box<dyn Fn(&&str) -> bool> {
    let re = Regex::new("(\\d+)(cm|in)").unwrap();
    Box::new(move |s| {
        re.captures(s).map_or(false, |captures| {
            captures[1].parse().map_or(false, |v| {
                match &captures[2] {
                    "cm" => 150 <= v && v <= 193,
                    "in" => 59 <= v && v <= 76,
                    _ => false
                }
            })
        })
    })
}

fn matches(pattern: &str) -> Box<dyn Fn(&&str) -> bool> {
    let re = Regex::new(pattern).unwrap();
    Box::new(move |s| {
        re.is_match(s)
    })
}

fn main() {
    let stdin = io::stdin();

    let mut contents = String::new();

    let requirements: Vec<(&str, Box<dyn Fn(&&str) -> bool>)>= vec!(
        ("byr", date_between(1920, 2002)),
        ("iyr", date_between(2010, 2020)),
        ("eyr", date_between(2020, 2030)),
        ("hgt", height()),
        ("hcl", matches("^#[0-9a-f]{6}$")),
        ("ecl", matches("^amb|blu|brn|gry|grn|hzl|oth$")),
        ("pid", matches("^\\d{9}$"))
    );

    stdin.lock().read_to_string(&mut contents).unwrap();

    let result = contents
        .split("\n\n")
        .map(|pcontents| {
            pcontents.split_ascii_whitespace().map(|fcontent| {
                let colon = fcontent.find(":").unwrap();
                (&fcontent[..colon], &fcontent[(colon+1)..])
            }).collect::<HashMap<&str, &str>>()
        })
        .filter(|passport| {
            requirements.iter().all(|(k, r)| passport.get(k).map_or(false, r))
        })
        .count();

    println!("{}", result)
}
