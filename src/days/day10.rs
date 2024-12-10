use std::fs::read_to_string;
use rustc_hash::FxHashSet;

use crate::etc::{Matrix, Pos};
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day10.txt").unwrap();
    let grid = Matrix::map_digits(&input);

    let scores = grid.find_all(0).map(|p| trailhead_score(&grid, p));
    let (sol1, sol2) = scores.reduce(|prev, next| (prev.0 + next.0, prev.1 + next.1)).unwrap();

    (Solution::from(sol1), Solution::from(sol2))
}

fn trailhead_score(grid: &Matrix<u8>, start: Pos) -> (usize, usize) {
    let mut uniq = FxHashSet::default();
    let total = dfs(grid, start, &mut uniq);
    (uniq.len(), total)
}

fn dfs(grid: &Matrix<u8>, pos: Pos, unique: &mut FxHashSet<Pos>) -> usize {
    let val = grid[pos];
    let next = neighbors(grid, pos, val + 1);
    if val == 8 {
        unique.extend(next.iter());
        return next.len();
    }
    next.into_iter().map(|p| dfs(grid, p, unique)).sum()
}

fn neighbors(grid: &Matrix<u8>, pos: Pos, expect: u8) -> Vec<Pos> {
    pos.neighbors().into_iter()
       .filter(|&p| grid.get_or(p, u8::MAX) == expect)
       .collect()
}
