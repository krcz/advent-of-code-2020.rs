use std::io;
use std::io::prelude::*;
use std::iter::Iterator;
use bit_vec::BitVec;

struct TriSpace {
    xn: usize,
    yn: usize,
    ym: usize,
    zm: usize,
    round: i32,
    data: BitVec
}

impl TriSpace {
    fn new(xn: usize, yn: usize, round: u32) -> TriSpace {
        let ym = xn + 2 * round as usize;
        let zm = ym * (yn + 2 * round as usize);
        let size = zm * (1 + 2 * round as usize);
        let mut data = BitVec::with_capacity(size);
        data.grow(size, false);
        TriSpace {xn, yn, ym, zm, round: round as i32, data}
    }

    fn minx(&self) -> i32 {
        -self.round
    }


    fn miny(&self) -> i32 {
        -self.round
    }


    fn minz(&self) -> i32 {
        -self.round
    }

    fn maxx(&self) -> i32 {
        self.round + self.xn as i32 - 1
    }

    fn maxy(&self) -> i32 {
        self.round + self.yn as i32 - 1
    }

    fn maxz(&self) -> i32 {
        self.round
    }

    fn xmap(&self, x: i32) -> Option<usize> {
        let xx = x + self.round as i32;
        if xx >= 0 && xx < self.xn as i32 + 2 * self.round {
            Some(xx as usize)
        } else {
            None
        }
    }

    fn ymap(&self, y: i32) -> Option<usize> {
        let yy = y + self.round as i32;
        if yy >= 0 && yy < self.yn as i32 + 2 * self.round {
            Some(yy as usize)
        } else {
            None
        }
    }

    fn zmap(&self, z: i32) -> Option<usize> {
        let zz = z + self.round as i32;
        if zz >= 0 && zz < 1 + 2 * self.round {
            Some(zz as usize)
        } else {
            None
        }
    }

    fn get(&self, x: i32, y: i32, z: i32) -> bool {
        //println!("get {} {} {}", x, y, z);
        if let Some(xx) = self.xmap(x) {
            if let Some(yy) = self.ymap(y) {
                if let Some(zz) = self.zmap(z) {
                    return self.data.get(xx + yy * self.ym + zz * self.zm).unwrap();
                }
            }
        }
        return false;
    }

    fn set(&mut self, x: i32, y: i32, z: i32, value: bool) {
        //println!("set {} {} {}", x, y, z);
        let xx = self.xmap(x).unwrap();
        let yy = self.ymap(y).unwrap();
        let zz = self.zmap(z).unwrap();
        self.data.set(xx + yy * self.ym + zz * self.zm, value);
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item=bool> + 'a {
        self.data.iter()
    }

}



struct Neighbourhood<'a> {
    board: &'a TriSpace,
    x: i32,
    y: i32,
    z: i32
}

impl<'a> Neighbourhood<'a> {
    fn iter(&'a self) -> impl Iterator<Item = bool> + 'a {
        (0 as i32 .. 27 as i32).filter_map(move |k: i32| -> Option<bool> {
            if k == 13 {
                None
            } else {
                Some(self.board.get(self.x - 1 + k % 3, self.y - 1 + k / 3 % 3, self.z - 1 + k / 9))
            }
        })
    }
}

fn main() {
    let stdin = io::stdin();

    let mut flat_board: Vec<Vec<char>> = stdin.lock().lines().map(|line| {
        line.unwrap().trim().chars().collect()
    }).collect();

    let yn = flat_board.len();
    let xn = flat_board[0].len();

    let mut board = TriSpace::new(xn, yn, 0);

    for x in 0..xn {
        for y in 0..yn {
            board.set(x as i32, y as i32, 0, flat_board[y][x] == '#')
        }
    }

    let max_round = 6;

    for round in 1..=max_round {
        let mut nb = TriSpace::new(xn, yn, round);

        for x in nb.minx()..=nb.maxx() {
            for y in nb.miny()..=nb.maxy() {
                for z in nb.minz()..=nb.maxz() {
                    let occ: u32 = Neighbourhood { board: &board, x, y, z}.iter().filter(|&b| b).count() as u32;
                    nb.set(x, y, z, if board.get(x, y, z) { occ == 2 || occ == 3 } else { occ == 3 })
                }
            }
        }

        println!("After round {}", round);
        for y in nb.miny()..=nb.maxy() {
            println!("{}", (nb.minx()..=nb.maxx()).map(|x| if nb.get(x, y, 0) {'#'} else {'.'}).collect::<String>());
        }
        println!();

        board = nb;
    }

    println!("{}", board.iter().filter(|&b| b).count());
}
