use std::io;
use std::io::prelude::*;
use std::iter::Iterator;

fn calc(board: &Vec<Vec<bool>>, ds: usize, rs: usize) -> u32 {
    let n = board[0].len();
    let m = board.len();
    (0..(m + ds - 1)/ds).map(|i| board[ds * i][(rs*i)%n] as u32).sum()
}

fn main() {
    let stdin = io::stdin();

    let board: Vec<Vec<bool>> = stdin.lock().lines().map(|line| -> Vec<bool> {
        line.unwrap().trim().chars().map(|c| c == '#').collect()
    }).collect();

    let n = board[0].len();
    assert!(board.iter().all(|row| row.len() == n));

    let result = (calc(&board, 1, 1) as u64) *
        (calc(&board, 1, 3) as u64) *
        (calc(&board, 1, 5) as u64) *
        (calc(&board, 1, 7) as u64) *
        (calc(&board, 2, 1) as u64);

    println!("{}", result);
}
