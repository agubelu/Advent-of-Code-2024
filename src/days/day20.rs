use std::fs::read_to_string;

use lazy_static::lazy_static;
use itertools::Itertools;
use rayon::prelude::*;

use crate::etc::{Pos, Matrix};
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

static NEIGHBORS_P1: [Pos; 4] = [Pos::new(2, 0), Pos::new(-2, 0), Pos::new(0, 2), Pos::new(0, -2)];
lazy_static! {
    static ref NEIGHBORS_P2: Vec<Pos> = (-20..=20).cartesian_product(-20..=20)
        .map(|(x, y)| Pos::new(x, y))
        .filter(move |&pos| {
            let dist = pos.x.abs() + pos.y.abs();
            dist >= 2 && dist <= 20
        }).collect();
}

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day20.txt").unwrap();
    let grid = Matrix::from_str(&input);

    let (dist_grid, path) = navigate_path(&grid);

    let sol1 = find_cheats(&dist_grid, &path, &NEIGHBORS_P1);
    let sol2 = find_cheats(&dist_grid, &path, &NEIGHBORS_P2);

    (Solution::from(sol1), Solution::from(sol2))
}

fn find_cheats(dists: &Matrix<i32>, path: &[Pos], radius: &[Pos]) -> usize {
    path.par_iter().flat_map_iter(|&p| cheats_in_point(dists, p, radius)).count()
}

fn cheats_in_point<'a>(dists: &'a Matrix<i32>, pos: Pos, radius: &'a[Pos]) -> impl 'a + Iterator<Item = i32> {
    let dist = dists[pos];
    radius.iter().map(move |p| pos + p) // Posible outbound positions
          .filter(move |p| dists.get_or(*p, i32::MAX) < dist) // Don't go backwards in the path
          .map(move |p| dist - dists[p] - p.manhattan_dist(&pos)) // Compute cheat distance
          .filter(|&d| d >= 100)
}

fn navigate_path(grid: &Matrix<char>) -> (Matrix<i32>, Vec<Pos>) {
    // Navigate the path backwards to store distance to the end for each point in the maze
    let mut dist_grid = Matrix::new(grid.width(), grid.height(), i32::MAX);
    let mut visited = vec![];
    let mut pos_opt = grid.find('E');

    while let Some(pos) = pos_opt {
        dist_grid[pos] = visited.len() as i32;
        visited.push(pos);
        pos_opt = pos.neighbors().into_iter()
            .filter(|&p| grid[p] != '#' && dist_grid[p] == i32::MAX)
            .exactly_one().ok();
    }
    (dist_grid, visited)
}
