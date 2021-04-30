use std::io;
use std::io::prelude::*;

use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let stdin = io::stdin();

    let v: Vec<u64> = stdin.lock().lines().next().unwrap().unwrap().split(",").map(|s| s.parse().unwrap()).collect();

    let mut h: Vec<Option<u64>> = (0 .. 30_000_000).map(|_| None).collect();
    v.iter().dropping_back(1).enumerate().for_each(|(i, &v)| h[v as usize] = Some((i + 1) as u64));

    //let mut h: HashMap<u64, u64> = v.iter().dropping_back(1).enumerate().map(|(i, &v)| (v, (i + 1) as u64)).collect();

    let gen_stream = itertools::unfold((&mut h, v.len() as u64, *v.last().unwrap()), |(h, ind, prev)| {
        let v = h.get(*prev as usize).unwrap().map_or::<u64, _>(0, |lind| *ind - lind);
        //h.insert(*prev, *ind);
        h[*prev as usize] = Some(*ind);
        *ind += 1;
        *prev = v;
        Some(v)
    });

    let mut stream = v.iter().cloned().chain(gen_stream);

    //println!("{:?}", stream.take(10).collect::<Vec<_>>());
    //println!("{:?}", h);
    println!("{}", stream.nth(30_000_000 - 1).unwrap());

}
