use std::fs::read_to_string;

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{Solution, SolutionPair};
use crate::etc::{Matrix, Pos};

///////////////////////////////////////////////////////////////////////////////

type AntennaMap = FxHashMap<char, Vec<Pos>>;

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day08.txt").unwrap();
    let grid = Matrix::from_str(&input);
    let antennas = find_antennas(&grid);

    let sol1 = find_all_antinodes(&antennas, &grid, false);
    let sol2 = find_all_antinodes(&antennas, &grid, true);

    (Solution::from(sol1), Solution::from(sol2))
}

fn find_antennas(grid: &Matrix<char>) -> AntennaMap {
    let mut positions: AntennaMap = AntennaMap::default();

    for (pos, ch) in grid.enumerate().filter(|x| x.1 != '.') {
        positions.entry(ch)
            .and_modify(|ls| ls.push(pos))
            .or_insert(vec![pos]);
    }

    positions
}

fn find_all_antinodes(map: &AntennaMap, grid: &Matrix<char>, all: bool) -> usize {
    let antinodes: FxHashSet<Pos> = map.values().flat_map(|antennas| {
        antennas.iter().tuple_combinations().flat_map(|(a, b)| pair_antinodes(*a, *b, grid, all))
    }).collect();
    antinodes.into_iter().filter(|&pos| grid.is_in_bounds(pos)).count()
}

fn pair_antinodes(a: Pos, b: Pos, grid: &Matrix<char>, all: bool) -> Vec<Pos> {
    let diff = b - a;
    if !all {
        vec![b + diff, a - diff]
    } else {
        (0..).map(|i| b + diff * i).take_while(|&p| grid.is_in_bounds(p))
            .chain((1..).map(|i| b - diff * i).take_while(|&p| grid.is_in_bounds(p)))
            .collect()
    }
}
