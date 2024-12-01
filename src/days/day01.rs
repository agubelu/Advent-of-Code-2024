use std::fs::read_to_string;
use itertools::*;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day01.txt").unwrap();
    let (mut ls1, mut ls2) = parse(&input);

    ls1.sort();
    ls2.sort();
    let counts = ls2.iter().counts();

    let sol1: i32 = ls1.iter().zip(ls2.iter())
        .map(|(i1, i2)| (i1 - i2).abs())
        .sum();

    let sol2: i32 = ls1.iter()
        .map(|i| i * counts.get(&i).copied().unwrap_or(0) as i32)
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (mut ls1, mut ls2) = (vec![], vec![]);
    for line in input.lines() {
        let (i1, i2) = line.split_whitespace().collect_tuple().unwrap();
        ls1.push(i1.parse().unwrap());
        ls2.push(i2.parse().unwrap());
    }
    (ls1, ls2)
}
