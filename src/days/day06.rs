use std::fs::read_to_string;

use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::FxHashSet;

use crate::etc::{Matrix, Pos};
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

static DIRECTIONS: [Pos; 4] = [Pos::const_up(), Pos::const_right(), Pos::const_down(), Pos::const_left()];
struct NavResult { visited: FxHashSet<(Pos, usize)>, looped: bool }

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day06.txt").unwrap();
    let grid = Matrix::from_str(&input);

    let (sol1, sol2) = simulate_path(&grid);
    (Solution::from(sol1), Solution::from(sol2))
}

fn simulate_path(grid: &Matrix<char>) -> (usize, usize) {
    let start = grid.enumerate().find(|x| x.1 == '^').unwrap().0;
    let result = navigate(grid, start, 0);
    let unique_positions = result.visited.into_iter().map(|x| x.0).unique().collect_vec();

    let loops = unique_positions.par_iter().copied()
        .filter(|&pos| {
            let mut grid2 = grid.clone();
            grid2[pos] = '#';
            navigate(&grid2, start, 0).looped
        }).count();

    (unique_positions.len(), loops)
}

fn navigate(grid: &Matrix<char>, mut pos: Pos, mut dir: usize) -> NavResult {
    // Navigates the map from the provided position and direction. Returns upon exiting bounds or looping.
    let mut visited = FxHashSet::default();

    while grid.is_in_bounds(pos) && !visited.contains(&(pos, dir)) {
        visited.insert((pos, dir));
        let next = pos + DIRECTIONS[dir];
        match grid.get_or_dot(next) {
            '#' => dir = (dir + 1) % 4,
             _  => pos = next,
        }
    }

    NavResult{ visited, looped: grid.is_in_bounds(pos) }
}
