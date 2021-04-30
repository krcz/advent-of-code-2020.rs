use std::io;
use std::io::prelude::*;
use std::iter::Iterator;
use regex::Regex;

fn rot(x: i64, y: i64, d: u8) -> i64 {
    x * (1 ^ (d & 1)) as i64 * (1 - (d & 2) as i64) + y * (d & 1) as i64 * ((d & 2) as i64 - 1)
}

fn main() {
    let stdin = io::stdin();

    let re = Regex::new(r#"(\w)(\d+)"#).unwrap();

    let (x, y, _, _) = stdin
        .lock()
        .lines()
        .map(|line| {
            let uline = line.unwrap();
            let captures = re.captures(&uline).unwrap();
            let n = captures[2].parse::<i64>().unwrap();
            (captures[1].chars().next().unwrap(), n)
        })
        .fold((0 as i64, 0 as i64, 10 as i64, -1 as i64), |(x, y, wx, wy), (c, n)| {
            println!("{} {} {} {}", x, y, wx, wy);
            println!("{}{}", c, n);
            let dir = (n / 90) as u8;
            match c {
                'N' => (x, y, wx, wy - n),
                'S' => (x, y, wx, wy + n),
                'E' => (x, y, wx + n, wy),
                'W' => (x, y, wx - n, wy),
                'L' => (x, y, rot(wx, wy, 4 - dir), rot(wy, -wx, 4 - dir)),
                'R' => (x, y, rot(wx, wy, dir), rot(wy, -wx, dir)),
                'F' => (x + n * wx, y + n * wy, wx, wy),
                _ => panic!("Unknown command: {}", c)
            }
        });

    println!("{} {} {}", x, y, x.abs() + y.abs());
}
