use std::fs::read_to_string;
use rustc_hash::FxHashSet;
use crate::{Solution, SolutionPair};
use crate::etc::{Matrix, Pos};

///////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Default)]
struct RegionData {
    area: usize,
    perimeter: usize,
    sides: usize,
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day12.txt").unwrap();
    let grid = Matrix::from_str(&input);

    let regions = find_regions(&grid);

    let sol1: usize = regions.iter().map(|r| r.area * r.perimeter).sum();
    let sol2: usize = regions.iter().map(|r| r.area * r.sides).sum();

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn find_regions(grid: &Matrix<char>) -> Vec<RegionData> {
    let mut regions = vec![];
    let mut visited = FxHashSet::default();

    for (pos, _) in grid.enumerate() {
        if !visited.contains(&pos) {
            regions.push(explore_region(grid, pos, &mut visited));
        }
    }

    regions
}

fn explore_region(grid: &Matrix<char>, pos: Pos, visited: &mut FxHashSet<Pos>) -> RegionData {
    let ident = grid[pos];
    let mut to_visit = FxHashSet::default();
    to_visit.insert(pos);
    let mut region_data = RegionData::default();

    // BFS the region until we run out of positions to visit
    while !to_visit.is_empty() {
        let mut new_to_visit = FxHashSet::default();
        for v in to_visit {
            visited.insert(v);
            let (perims, corners) = analyze_tile(v, grid, ident);
            region_data.area += 1;
            region_data.perimeter += perims;
            region_data.sides += corners;

            let neighbors = v.neighbors().into_iter().filter(|&p| grid.get_or_dot(p) == ident && !visited.contains(&p));
            new_to_visit.extend(neighbors);
        }
        to_visit = new_to_visit;
    }
    region_data
}

fn analyze_tile(pos: Pos, grid: &Matrix<char>, ch: char) -> (usize, usize) {
    // Check whether the four cardinal neighbors of this position are boundaries
    let [u, d, l, r] = pos.neighbors().map(|p| grid.get_or_dot(p) != ch);

    // Check for all corners. A corner occurs when two perpendicular cardinal directions
    // are boundaries (exterior corner) or when they're not, but their diagonal is (interior corner)
    let ul = u && l || !u && !l && grid.get_or_dot(pos.up().left()) != ch;
    let dl = d && l || !d && !l && grid.get_or_dot(pos.down().left()) != ch;
    let dr = d && r || !d && !r && grid.get_or_dot(pos.down().right()) != ch;
    let ur = u && r || !u && !r && grid.get_or_dot(pos.up().right()) != ch;

    let perim = [u, d, l, r].map(|x| x as usize).into_iter().sum();
    let corners = [ul, dl, ur, dr].map(|x| x as usize).into_iter().sum();
    (perim, corners)
}
