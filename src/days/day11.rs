use std::fs::read_to_string;
use rustc_hash::FxHashMap;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type StoneCounter = FxHashMap<u64, u64>;

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day11.txt").unwrap();
    let mut stones = parse(&input);

    for _ in 0..25 { evolve(&mut stones); }
    let sol1 = stones.values().sum();

    for _ in 0..50 { evolve(&mut stones); }
    let sol2 = stones.values().sum();

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn evolve(stones: &mut StoneCounter) {
    let mut new_stones = StoneCounter::default();

    for (&value, &count) in stones.iter() {
        let n_digits = value.checked_ilog10().unwrap_or(0) + 1;
        match value {
            0 => *new_stones.entry(1).or_insert(0) += count,
            x if n_digits % 2 == 0 => {
                let half = 10u64.pow(n_digits / 2);
                *new_stones.entry(x / half).or_insert(0) += count;
                *new_stones.entry(x % half).or_insert(0) += count;
            },
            _ => *new_stones.entry(value * 2024).or_insert(0) += count
        };
    }

    *stones = new_stones;
}

fn parse(input: &str) -> StoneCounter {
    let mut counter = StoneCounter::default();
    input.split_whitespace().for_each(|v| *counter.entry(v.parse().unwrap()).or_insert(0) += 1);
    counter
}
