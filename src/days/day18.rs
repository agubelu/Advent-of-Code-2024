use std::fs::read_to_string;

use pathfinding::directed::astar::astar;
use rustc_hash::FxHashSet;

use crate::etc::Pos;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

const GRID_SIZE: i32 = 71;

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day18.txt").unwrap();
    let bytes = parse_points(&input);

    let sol1 = shortest_path(&bytes, 1024).unwrap();
    let byte = find_clogging_byte(&bytes);
    let sol2 = format!("{},{}", byte.x, byte.y);

    (Solution::from(sol1), Solution::from(sol2))
}

fn shortest_path(bytes: &[Pos], until: usize) -> Option<i32> {
    let start = Pos::new(0, 0);
    let end = Pos::new(GRID_SIZE - 1, GRID_SIZE - 1);

    // Build the grid up to the desired byte
    let blocked: FxHashSet<_> = bytes.iter().copied().take(until).collect();

    astar(
        &start,
        |&pos| neighbors(pos, &blocked),
        |&pos| pos.manhattan_dist(&end),
        |&pos| pos == end
    ).map(|sol| sol.1)
}

fn find_clogging_byte(bytes: &[Pos]) -> Pos {
    // Find the first index that clogs up the maze with binary search
    let (mut lo, mut hi) = (1025, bytes.len() - 1);
    let mut res = lo;
    while lo <= hi {
        let i = (hi + lo) / 2;
        match shortest_path(bytes, i+1) {
            Some(_) => lo = i + 1, // Too low
            None => { // Too high, but save this index
                res = i;
                hi = i - 1;
            }
        }
    }
    bytes[res]
}

fn neighbors(pos: Pos, blocked: &FxHashSet<Pos>) -> impl '_ + IntoIterator<Item = (Pos, i32)> {
    pos.neighbors()
       .into_iter()
       .filter(|p| p.x >= 0 && p.x < GRID_SIZE && p.y >= 0 && p.y < GRID_SIZE && !blocked.contains(p))
       .map(|p| (p, 1))
}

fn parse_points(input: &str) -> Vec<Pos> {
    input.lines()
         .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Pos::new(x.parse().unwrap(), y.parse().unwrap())
         }).collect()
}
