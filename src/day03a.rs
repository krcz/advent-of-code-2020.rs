use std::io;
use std::io::prelude::*;
use std::iter::Iterator;

fn main() {
    let stdin = io::stdin();

    let board: Vec<Vec<bool>> = stdin.lock().lines().map(|line| -> Vec<bool> {
        line.unwrap().trim().chars().map(|c| c == '#').collect()
    }).collect();

    let n = board[0].len();
    assert!(board.iter().all(|row| row.len() == n));
    let m = board.len();

    let result: u32 = (0..m).map(|i| board[i][(3*i)%n] as u32).sum();

    println!("{}", result);
}
