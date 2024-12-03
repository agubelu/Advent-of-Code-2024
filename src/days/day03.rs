use std::fs::read_to_string;
use regex::Regex;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day03.txt").unwrap();
    let re = Regex::new(r#"(do|don't|mul)\((\d+,\d+)?\)"#).unwrap();

    let (mut sol1, mut sol2) = (0, 0);
    let mut enabled = 1;

    for cap in re.captures_iter(&input) {
        match &cap[1] {
            "do" => enabled = 1,
            "don't" => enabled = 0,
            "mul" => {
                let mult: u64 = cap[2].split(',').map(|x| x.parse::<u64>().unwrap()).product();
                sol1 += mult;
                sol2 += mult * enabled;
            },
            _ => unreachable!(),
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}
