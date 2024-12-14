use std::fs::read_to_string;
use std::cmp::Ordering::*;

use itertools::Itertools;
use rustc_hash::FxHashSet;
use sscanf::sscanf;

use crate::etc::Pos;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

struct Robot {
    position: Pos,
    velocity: Pos,
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day14.txt").unwrap();
    let mut robots = input.lines().map(parse_robot).collect_vec();

    let mut i = 0;

    while i < 100 {
        i += 1;
        advance_robots(&mut robots);
    }
    let sol1 = safety_factor(&robots);

    while !has_christmas_tree(&robots) {
        i += 1;
        advance_robots(&mut robots);
    }

    (Solution::from(sol1), Solution::from(i))
}

fn safety_factor(robots: &[Robot]) -> usize {
    let mut quadrants = [0, 0, 0, 0];
    robots.iter().filter_map(robot_quadrant).for_each(|i| quadrants[i] += 1);
    quadrants.into_iter().product()
}

fn robot_quadrant(robot: &Robot) -> Option<usize> {
    match (robot.position.x.cmp(&(WIDTH / 2)), robot.position.y.cmp(&(HEIGHT / 2))) {
        (Less, Less) => Some(0),
        (Greater, Less) => Some(1),
        (Less, Greater) => Some(2),
        (Greater, Greater) => Some(3),
        _ => None
    }
}

fn has_christmas_tree(robots: &[Robot]) -> bool {
    // Find a straight horizontal line
    let minimum_length = 20;
    let mut robots_per_line = [0; HEIGHT as usize];
    for robot in robots {
        robots_per_line[robot.position.y as usize] += 1;
    }

    // Only check for an actual consecutive line if any row has enough robots
    if let Some((y, _)) = robots_per_line.into_iter().find_position(|cnt| *cnt >= minimum_length) {
        let set: FxHashSet<Pos> = robots.iter().map(|r| r.position).collect();
        let mut line = 0;
        for x in 0..WIDTH {
            if set.contains(&(x, y as i32).into()) {
                line += 1;
            } else {
                line = 0;
            }
            if line == minimum_length { return true }
        }
    }
    false
}

fn advance_robots(robots: &mut[Robot]) {
    for robot in robots.iter_mut() {
        robot.position += robot.velocity;
        robot.position.x = robot.position.x.rem_euclid(WIDTH);
        robot.position.y = robot.position.y.rem_euclid(HEIGHT);
    }
}

fn parse_robot(line: &str) -> Robot {
    let (px, py, vx, vy) = sscanf!(line, "p={i32},{i32} v={i32},{i32}").unwrap();
    Robot { position: Pos::new(px, py), velocity: Pos::new(vx, vy) }
}
