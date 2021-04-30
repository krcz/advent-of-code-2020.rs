use std::io;
use std::io::prelude::*;
use std::iter::Iterator;

struct Neighbourhood<'a> {
    board: &'a Vec<Vec<char>>,
    i: usize,
    j: usize
}

impl<'a> Neighbourhood<'a> {
    fn iter(&'a self) -> impl Iterator<Item = char> + 'a {
        (0 as usize .. 9 as usize).filter_map(move |k: usize| -> Option<char> {
            if k == 4 {
                None
            } else {
                let mut is = self.i + k/3;
                let mut js = self.j + k%3;
                loop {
                    if is <= 0 || is > self.board.len() || js <= 0 || js > self.board[is - 1].len() {
                        break None
                    } else {
                        let c = self.board[is - 1][js - 1];
                        if c == '#' || c == 'L' {
                            break Some(c)
                        } else {
                            is = is - 1 + k / 3;
                            js = js - 1 + k % 3;
                        }
                    }
                }
            }
        })
    }
}

fn main() {
    let stdin = io::stdin();

    let mut board: Vec<Vec<char>> = stdin.lock().lines().map(|line| {
        line.unwrap().trim().chars().collect()
    }).collect();

    let n = board.len();
    let m = board[0].len();

    loop {
        let mut changed = false;

        let oldb = board.clone();

        for i in 0..n {
            for j in 0..m {
                let occ: usize = Neighbourhood {board: &oldb, i, j}.iter().filter(|c| *c == '#').count();

                //println!("{} {} {}", i, j, occ);

                if oldb[i][j] == 'L' && occ == 0 {
                    board[i][j] = '#';
                    changed = true;
                } else if oldb[i][j] == '#' && occ >= 5 {
                    board[i][j] = 'L';
                    changed = true;
                }
            }
        }

        //for row in &board {
        //    println!("{}", row.iter().collect::<String>())
        //}
        //println!();

        if !changed {
            break
        }
    }

    let occupied = board.iter().flat_map(|row| row.iter()).filter(|c| **c == '#').count();

    println!("{}", occupied);
}
