use dashmap::DashMap;
use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::FxHashSet;

use std::fs::read_to_string;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type SeqMap = DashMap<(i64, i64, i64, i64), i64>;

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day22.txt").unwrap();
    let seeds = input.lines().map(|x| x.parse().unwrap()).collect_vec();

    let sequences = SeqMap::default();

    let sol1: i64 = seeds.par_iter().map(|&x| advance(x, &sequences)).sum();
    let sol2 = sequences.iter().map(|x| *x.value()).max().unwrap();

    (Solution::from(sol1), Solution::from(sol2))
}

fn advance(mut x: i64, sequences: &SeqMap) -> i64 {
    let mut prev = x % 10;
    let mut seq = (0, 0, 0, 0);
    let mut seen = FxHashSet::default();

    for i in 0..2000 {
        x = next(x);
        let price = x % 10;
        seq = (price - prev, seq.0, seq.1, seq.2);
        prev = price;

        if i >= 3 && !seen.contains(&seq) {
            seen.insert(seq);
            *sequences.entry(seq).or_insert(0) += price;
        }
    }

    x
}

fn next(mut x: i64) -> i64 {
    x ^= (x << 6) & 0xFFFFFF;
    x ^= (x >> 5) & 0xFFFFFF;
    (x ^ (x << 11)) & 0xFFFFFF
}
