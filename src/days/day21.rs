use std::fs::read_to_string;
use std::{iter, array};

use itertools::Itertools;
use lazy_static::lazy_static;
use rustc_hash::FxHashMap;

use crate::etc::Pos;
use crate::{Solution, SolutionPair};
use Input::*;

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Input {
    Right = 0,
    Left = 1,
    Up = 2,
    Down = 3,
    Act = 4,
}

struct RobotState {
    position: Vec<Input>,
    cache: FxHashMap<(usize, Input, Input), u64>,
}

// Position of the keys in the code keypad. A = 10.
static CODE_KEYPAD: [Pos; 11] = [
    Pos::new(1, 3), Pos::new(0, 2), Pos::new(1, 2),
    Pos::new(2, 2), Pos::new(0, 1), Pos::new(1, 1),
    Pos::new(2, 1), Pos::new(0, 0), Pos::new(1, 0),
    Pos::new(2, 0), Pos::new(2, 3),
];

// Position of the keys in the robot input keyboard. Order is the same as values of `Input`
static ROBOT_KEYPAD: [Pos; 5] = [
    Pos::new(2, 1), Pos::new(0, 1), Pos::new(1, 0), Pos::new(1, 1), Pos::new(2, 0),
];

lazy_static! {
    static ref ROBOT_MOVES: [[Vec<Input>; 5]; 5] = build_input_movements();
    static ref CODE_MOVES: [[Vec<Input>; 11]; 11] = build_keypad_movements();
}

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day21.txt").unwrap();
    let codes = input.lines().collect_vec();

    let sol1 = codes.iter().map(|code| process_code(code, 2)).sum();
    let sol2 = codes.iter().map(|code| process_code(code, 25)).sum();

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn process_code(code: &str, num_robots: usize) -> u64 {
    // State for the main code keypad
    let mut cur_button = 10; // Start on the `A` button
    let mut total_presses = 0;

    // State for the chain of robots
    let mut state = RobotState {
        position: vec![Input::Act; num_robots],
        cache: FxHashMap::default(),
    };

    for ch in code.chars() {
        let next_button = ch.to_digit(10).unwrap_or(10) as usize; // Last one is for the A button
        let mut paths = vec![CODE_MOVES[cur_button][next_button].to_owned()];

        // Try both orderings of the same path if possible
        if !matches!((cur_button, next_button), (0|10, 1|4|7) | (1|4|7, 0|10)) {
            paths.push(paths[0].clone());
            paths[1].reverse();
        }

        let optimal = paths.into_iter().map(|path| {
            let mut this_path = 0;

            for input in path {
                this_path += propagate_input(input, num_robots, &mut state);
            }
            this_path + propagate_input(Input::Act, num_robots, &mut state)
        }).min().unwrap();

        total_presses += optimal;
        cur_button = next_button;
    }

    total_presses * code.trim_end_matches('A').parse::<u64>().unwrap()
}

fn propagate_input(target: Input, i: usize, state: &mut RobotState) -> u64 {
    if i == 0 {  // If there are no more robots after this one, simply make the required input
        return 1;
    }

    let current = state.position[i-1];
    if let Some(&res) = state.cache.get(&(i, current, target)) {
        state.position[i-1] = target; // We have to move the robot to the new position
        return res;
    }

    // Try both orderings of the same path if possible
    let mut paths = vec![ROBOT_MOVES[current as usize][target as usize].to_owned()];
    if current != Input::Left && target != Input::Left {
        paths.push(paths[0].clone());
        paths[1].reverse();
    }

    let total_presses = paths.into_iter().map(|path| {
        let mut this_path = 0;
        for input in path {
            this_path += propagate_input(input, i-1, state);
        }
        this_path + propagate_input(Input::Act, i-1, state)
    }).min().unwrap();

    state.position[i-1] = target;
    state.cache.insert((i, current, target), total_presses);
    total_presses
}

///////////////////////////////////////////////////////////////////////////////

fn build_input_movements() -> [[Vec<Input>; 5]; 5] {
    array::from_fn(|from| array::from_fn(|to| {
        let mut v = inputs_required(ROBOT_KEYPAD[from], ROBOT_KEYPAD[to]);
        // If going from/to the left key, make sure to do horizontal
        // movements first/last respectively, to avoid passing over the empty space
        if to == Input::Left as usize {
            v.sort_by(|a, b| b.cmp(a));
        } else {
            v.sort();
        }
        v
    }))
}

fn build_keypad_movements() -> [[Vec<Input>; 11]; 11] {
    array::from_fn(|from| array::from_fn(|to| {
        let mut v = inputs_required(CODE_KEYPAD[from], CODE_KEYPAD[to]);
        // If going from/to the bottom keys, make sure to do horizontal
        // movements last/first respectively, to avoid passing over the empty space
        if from == 0 || from == 10 {
            v.sort_by(|a, b| b.cmp(a));
        } else {
            v.sort();
        }
        v
    }))
}

fn inputs_required(from: Pos, to: Pos) -> Vec<Input> {
    let vert = if to.y > from.y { Down } else { Up };
    let hor = if to.x > from.x { Right } else { Left };
    iter::repeat(vert).take((to.y - from.y).abs() as usize)
        .chain(iter::repeat(hor).take((to.x - from.x).abs() as usize))
        .collect()
}
