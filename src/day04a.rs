use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::iter::Iterator;

fn main() {
    let stdin = io::stdin();

    let mut contents = String::new();

    let required_fields = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

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
            required_fields.iter().all(|f| passport.contains_key(f))
        })
        .count();

    println!("{}", result)
}
