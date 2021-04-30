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
                let is = self.i + k/3;
                let js = self.j + k%3;
                if 0 < is && is <= self.board.len() && 0 < js && js <= self.board[is - 1].len() {
                    Some(self.board[is - 1][js - 1])
                } else { None }
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
                } else if oldb[i][j] == '#' && occ >= 4 {
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
