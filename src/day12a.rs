use std::io;
use std::io::prelude::*;
use std::iter::Iterator;
use regex::Regex;

fn main() {
    let stdin = io::stdin();

    let re = Regex::new(r#"(\w)(\d+)"#).unwrap();

    let (x, y, dir) = stdin
        .lock()
        .lines()
        .map(|line| {
            let uline = line.unwrap();
            let captures = re.captures(&uline).unwrap();
            let n = captures[2].parse::<i64>().unwrap();
            (captures[1].chars().next().unwrap(), n)
        })
        .fold((0 as i64, 0 as i64, 1 as u8), |(x, y, dir), (c, n)| {
            //println!("{} {} {}", x, y, dir);
            match c {
                'N' => (x, y - n, dir),
                'S' => (x, y + n , dir),
                'E' => (x - n, y, dir),
                'W' => (x + n, y, dir),
                'L' => (x, y, (dir + 4 - (n / 90) as u8)%4),
                'R' => (x, y, (dir + (n / 90) as u8)%4),
                'F' => (x + (dir & 1) as i64 * (n - (dir & 2) as i64 * n),
                        y + (1 ^ (dir & 1)) as i64 * ((dir & 2) as i64 * n - n),
                        dir),
                    _ => panic!("Unknown command: {}", c)
            }
        });

    println!("{} {} {}", x, y, x.abs() + y.abs());
}
