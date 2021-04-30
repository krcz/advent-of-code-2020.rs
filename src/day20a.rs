use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;

use itertools::Itertools;

struct Tile {
    id: u64,
    sides: Vec<(u64, u64)>,
    flipped_sides: Vec<(u64, u64)>,
    data: Vec<Vec<char>>
}

impl Tile {
    // takes left and top sides requirements and returns (right, bottom) values for fits
    fn fit<'a>(&'a self, left_req: Option<u64>, top_req: Option<u64>) -> impl Iterator<Item=(u64, u64, u8)> + 'a {
        let left_reqc = left_req.clone();
        let top_reqc = top_req.clone();
        let fits = move |sides: &'a Vec<(u64, u64)>| {
            let left_reqcc = left_reqc.clone();
            let top_reqcc = top_reqc.clone();
            sides.iter().cycle().tuple_windows().take(4).enumerate().filter_map(move |(i, (left, top, right, bottom))| {
                if left_reqcc.map_or(true, |req| req == left.0) && top_reqcc.map_or(true, |req| req == top.0) {
                    Some((right.1, bottom.1, i as u8))
                } else {
                    None
                }
            })
        };

        fits(&self.sides)
            .chain(fits(&self.flipped_sides).map(|(r, b, i)| { (r, b, i | 4u8) }))
    }

    fn rotate(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    }

    fn get_board(&self, rotation) -> Vec<Vec<char>> {

    }
}

impl FromIterator<String> for Tile {
    fn from_iter<I: IntoIterator<Item=String>>(iiter: I) -> Self {
        let mut it = iiter.into_iter();
        let header = it.next().unwrap();
        assert!(header.starts_with("Tile ") && header.ends_with(":"), "Invalid header: {}", header);
        let id: u64 = header[5..(header.len() - 1)].parse().unwrap();

        let board: Vec<Vec<char>> = it.map(|v| v.chars().collect()).collect();
        let n = board.len();
        assert!(board.iter().all(|row| row.len() == n));

        fn get_bits(cit: impl Iterator<Item=char>) -> u64 {
            cit.fold(0, |acc, c| { (acc << 1) | if c == '#' { 1 } else { 0  } })
        }

        fn get_side(cit: impl DoubleEndedIterator<Item=char> + Clone) -> (u64, u64) {
            (get_bits(cit.clone()), get_bits(cit.clone().rev()))
        }

        let sides = vec!(
            get_side(board.iter().rev().map(|r| r[0])),
            get_side(board[0].iter().cloned()),
            get_side(board.iter().map(|r| r[n - 1])),
            get_side(board[n - 1].iter().rev().cloned())
        );

        let flipped_sides: Vec<(u64, u64)> = sides.iter().rev().map(|(u, v)| (*v, *u)).collect();

        Tile { id, sides, flipped_sides , data: board}
    }
}

fn fit_tiles(tiles: &Vec<Tile>, width: usize, height: usize, stack: &mut Vec<(usize, u64, u64)>) -> bool {
    println!("{:?}", stack.iter().map(|x| tiles[x.0].id).collect_vec());
    if stack.len() == tiles.len() {
        return true;
    }
    let k = stack.len();

    let left_req: Option<u64> = if k % width == 0 { None } else { stack.last().map(|el| el.1) };
    let top_req: Option<u64> = if k < width { None } else { stack.get(stack.len() - width).map(|el| el.2) };

    for (i, tile) in tiles.iter().enumerate() {
        if !stack.iter().any(|el| el.0 == i) {
            for f in tile.fit(left_req, top_req) {
                stack.push((i, f.0, f.1));
                if fit_tiles(tiles, width, height, stack) {
                    return true;
                }
                stack.pop();
            }
        }
    }
    return false;
}

fn main() {
    let stdin = io::stdin();

    let tiles: Vec<Tile> = stdin.lock().lines().map(|l| l.unwrap()).batching(|it| {
        let mut pit = it.peekable();
        if pit.peek().is_none() {
            None
        } else {
            Some(pit.take_while(|l| !l.trim().is_empty()).collect())
        }
    }).collect();

    tiles.iter().for_each(|t| println!("Tile {} {:?}", t.id, t.sides.iter().map(|s| format!("{:b}/{:b}", s.0, s.1)).collect_vec()));

    let side_cnt = (0..).skip_while(|i| i*i < tiles.len()).next().unwrap();

    let mut stack: Vec<(usize, u64, u64)> = Vec::with_capacity(tiles.len());

    let success = fit_tiles(&tiles, side_cnt, side_cnt, &mut stack);
    assert!(success);

    let arrangement: Vec<u64> = stack.iter().map(|(i, _, _)| tiles[*i].id).collect();

    println!("{:?}", arrangement);
    println!("{}", arrangement[0] * arrangement[side_cnt - 1] * arrangement[arrangement.len() - side_cnt] * arrangement[arrangement.len() - 1]);

}
