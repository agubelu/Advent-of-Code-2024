use std::cmp::Ordering;
use std::fs::read_to_string;

use itertools::*;
use rustc_hash::FxHashMap;

use crate::etc::consts::DOUBLE_NEWLINE;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type SortRules = FxHashMap<(u32, u32), Ordering>;
type Update = Vec<u32>;

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day05.txt").unwrap();
    let (rules_str, updates_str) = input.split_once(DOUBLE_NEWLINE).unwrap();

    let rules = parse_rules(rules_str);
    let updates = updates_str.lines().map(
        |line| line.split(',').map(|x| x.parse().unwrap()).collect_vec()
    ).collect_vec();

    let (mut sol1, mut sol2) = (0, 0);

    for updt in updates {
        let sorted = sort(&updt, &rules);
        let median = sorted[sorted.len() / 2];
        if sorted == updt {
            sol1 += median;
        } else {
            sol2 += median;
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}

fn sort(update: &Update, rules: &SortRules) -> Update {
    update.iter().copied()
          .sorted_by(|&a, &b| rules.get(&(a, b)).copied().unwrap_or(Ordering::Equal))
          .collect_vec()
}

fn parse_rules(rules_str: &str) -> SortRules {
    rules_str.lines().flat_map(|line| {
        let (a, b) = line.split('|').map(|x| x.parse().unwrap()).collect_tuple().unwrap();
        [((a, b), Ordering::Less), ((b, a), Ordering::Greater)]
    }).collect()
}
