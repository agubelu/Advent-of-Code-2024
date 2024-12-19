use std::fs::read_to_string;

use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::FxHashMap;

use crate::etc::DOUBLE_NEWLINE;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type Cache<'a> = FxHashMap<&'a str, u64>;

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day19.txt").unwrap();
    let (top, bottom) = input.split_once(DOUBLE_NEWLINE).unwrap();

    let patterns = top.split(", ").collect_vec();
    let designs = bottom.lines().collect_vec();

    let solved_designs: Vec<u64> = designs.par_iter()
        .map(|d| solve_design(d, &patterns, &mut Cache::default()))
        .collect();

    let sol1 = solved_designs.iter().filter(|&c| *c > 0).count();
    let sol2: u64 = solved_designs.iter().sum();

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_design<'a>(design: &'a str, patterns: &[&'a str], cache: &mut Cache<'a>) -> u64 {
    if design.is_empty() {
        return 1; // Design solved
    }
    if let Some(&res) = cache.get(design) {
        return res; // Cache hit
    }

    let combos: u64 = patterns.iter().copied()
        .filter(|&pat| pat.len() <= design.len() && &design[..pat.len()] == pat)
        .map(|pat| solve_design(&design[pat.len()..], patterns, cache))
        .sum();

    cache.insert(design, combos);
    combos
}
