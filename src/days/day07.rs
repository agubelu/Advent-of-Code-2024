use std::fs::read_to_string;
use itertools::Itertools;
use rayon::prelude::*;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

struct Equation {
    target: u64,
    values: Vec<u64>,
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day07.txt").unwrap();
    let equations = input.lines().map(parse_equation).collect_vec();

    let sol1 = calibration_results(&equations, false);
    let sol2 = calibration_results(&equations, true);

    (Solution::from(sol1), Solution::from(sol2))
}

fn calibration_results(equations: &[Equation], with_concat: bool) -> u64 {
    equations.par_iter()
        .filter(|eq| eq_solver(eq, eq.values[0], 1, with_concat))
        .map(|eq| eq.target).sum()
}

fn eq_solver(eq: &Equation, acc: u64, i: usize, concat: bool) -> bool {
    if acc > eq.target {
        // NB: this depends on the accumulated value increasing monotonically.
        // This does not happen if any value is zero, but it doesn't seem to be the case.
        return false;
    } else if i == eq.values.len() {
        // All values processed, check whether the result is the target value
        return acc == eq.target;
    }
    eq_solver(eq, acc + eq.values[i], i + 1, concat) ||
    eq_solver(eq, acc * eq.values[i], i + 1, concat) ||
    concat && eq_solver(eq, concat_numbers(acc, eq.values[i]), i + 1, concat)
}

fn parse_equation(line: &str) -> Equation {
    let (l, r) = line.split_once(": ").unwrap();
    let values = r.split(' ').map(|x| x.parse().unwrap()).collect();
    Equation { target: l.parse().unwrap(), values }
}

fn concat_numbers(a: u64, b: u64) -> u64 {
    a * 10_u64.pow(b.ilog10() + 1) + b
}
