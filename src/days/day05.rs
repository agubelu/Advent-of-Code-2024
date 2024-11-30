use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let _input = read_to_string("input/day01.txt").unwrap();

    let sol1 = 0u64;
    let sol2 = 0u64;

    (Solution::from(sol1), Solution::from(sol2))
}
