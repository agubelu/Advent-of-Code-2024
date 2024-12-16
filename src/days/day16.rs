use std::fs::read_to_string;

use itertools::Itertools;
use pathfinding::directed::astar::astar_bag;

use crate::etc::{Matrix, Pos};
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

static DIRS: [Pos; 4] = [Pos::const_right(), Pos::const_down(), Pos::const_left(), Pos::const_up()];

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct MazeState {
    position: Pos,
    direction: u8,
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day16.txt").unwrap();
    let grid = Matrix::from_str(&input);

    let (sol1, sol2) = find_min_paths(&grid);

    (Solution::from(sol1), Solution::from(sol2))
}

fn find_min_paths(grid: &Matrix<char>) -> (usize, usize) {
    let start = grid.find('S').unwrap();
    let end = grid.find('E').unwrap();
    let initial = MazeState { position: start, direction: 0 };

    let paths = astar_bag(
        &initial,
        |st| next_states(st, grid),
        |st| st.position.manhattan_dist(&end) as usize,
        |st| st.position == end
    ).unwrap();

    let cost = paths.1;
    let best_tiles_count = paths.0.flat_map(|path| path.into_iter().map(|s| s.position)).unique().count();

    (cost, best_tiles_count)
}

fn next_states(state: &MazeState, grid: &Matrix<char>) -> Vec<(MazeState, usize)> {
    [0, 1, 3].into_iter().filter_map(|turn| {
        let direction = (state.direction + turn) % 4;
        let position = state.position + DIRS[direction as usize];
        if grid.get_or(position, '#') != '#' {
            let cost = if turn == 0 { 1 } else { 1001 };
            Some((MazeState {position, direction }, cost))
        } else { None }
    }).collect()
}
