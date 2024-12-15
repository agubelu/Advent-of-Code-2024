use std::fs::read_to_string;

use itertools::Itertools;
use indexmap::IndexSet;

use crate::etc::{DOUBLE_NEWLINE, Pos, Matrix};
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day15.txt").unwrap();
    let (grid_str, dirs_str) = input.split_once(DOUBLE_NEWLINE).unwrap();
    let dirs = parse_dirs(dirs_str);
    let mut grid_p1 = Matrix::from_str(grid_str);
    let mut grid_p2 = make_p2_grid(&grid_p1);

    let sol1 = run_simulation(&mut grid_p1, &dirs);
    let sol2 = run_simulation(&mut grid_p2, &dirs);

    (Solution::from(sol1), Solution::from(sol2))
}

fn run_simulation(grid: &mut Matrix<char>, dirs: &[Pos]) -> i32 {
    let mut pos = grid.find('@').unwrap();
    grid[pos] = '.';

    for &dir in dirs {
        if let Some(boxes_hit) = check_movement(grid, pos, dir) {
            pos += dir; // Can move in this direction
            // boxes_hit are in reverse order w.r.t. direction, move the last ones first
            for box_pos in boxes_hit.into_iter().rev() {
                grid[box_pos + dir] = grid[box_pos];
                grid[box_pos] = '.';
            }
        }
    }

    box_scores(grid)
}

fn box_scores(grid: &Matrix<char>) -> i32 {
    grid.enumerate::<i32>()
        .filter(|(_, ch)| matches!(ch, 'O' | '['))
        .map(|(pos, _)| pos.x + 100 * pos.y)
        .sum()
}

fn check_movement(grid: &Matrix<char>, pos: Pos, dir: Pos) -> Option<IndexSet<Pos>> {
    // Checks whether the robot in the position `pos` can move in the direction `dir` (Some/None)
    // and the indices for all the boxes that must be moved in the specified direction.
    let mut hitboxes = IndexSet::new();
    let mut all_boxes_hit = IndexSet::new();
    hitboxes.insert(pos);

    loop {  // This will always eventually exit because going OOB returns a wall.
        let hitbox_neighbors = hitboxes.iter().map(|&pos| grid.get_or(pos + dir, '#')).collect_vec();

        if hitbox_neighbors.iter().any(|&ch| ch == '#') {
            return None; // Can't move, hit a wall.
        } else if hitbox_neighbors.iter().all(|&ch| ch == '.') {
            return Some(all_boxes_hit); // All hitboxes free, can move.
        }

        // Otherwise, we hit one or more boxes and need to update our hitboxes.
        hitboxes = hitboxes.into_iter().flat_map(|h| update_hitboxes(grid, h, dir)).collect();
        all_boxes_hit.extend(&hitboxes);
    }
}

fn update_hitboxes(grid: &Matrix<char>, hitbox: Pos, dir: Pos) -> Vec<Pos> {
    // Computes the new hitboxes after moving the provided `hitbox` in the specified direction.
    // If another box is hit, the box's coordinates are returned. Otherwise, the hitbox is lost.
    // hitbox + dir is always in bounds because otherwise a wall would have been hit.
    let next = hitbox + dir;
    match (grid[next], dir) {
        ('.', _) => vec![], // From here on, everthing else must be a box of some kind
        (_, d) if d.y == 0 => vec![next], // Horizontal movements simply propagate the hitbox in that direction
        ('O', _) => vec![next], // A simple box from part 1, hitbox is propagated
        ('[', _) => vec![next, next.right()], // Wide boxes being hit vertically,
        (']', _) => vec![next, next.left()],  // expand to the left-right as needed
        _ => unreachable!(),
    }
}

fn make_p2_grid(grid_p1: &Matrix<char>) -> Matrix<char> {
    let data_p2 = grid_p1.iter().flat_map(|ch| match ch {
        '#' => ['#', '#'],
        'O' => ['[', ']'],
        '.' => ['.', '.'],
        '@' => ['@', '.'],
         _  => unreachable!(),
    }).collect();
    Matrix::from_data(2 * grid_p1.width(), grid_p1.height(), data_p2)
}

fn parse_dirs(lines: &str) -> Vec<Pos> {
    lines.chars().filter(|ch| !ch.is_whitespace())
         .map(|ch| match ch {
             '^' => Pos::unit_up(),
             'v' => Pos::unit_down(),
             '<' => Pos::unit_left(),
             '>' => Pos::unit_right(),
              _  => unreachable!(),
         }).collect()
}
