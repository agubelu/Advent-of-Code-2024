use std::array;
use std::fs::read_to_string;
use itertools::iproduct;
use crate::etc::{DOUBLE_NEWLINE, Matrix};
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type Heights = [u8; 5];

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day25.txt").unwrap();
    let (locks, keys) = parse(&input);

    let sol1 = iproduct!(locks, keys)
        .filter(check_pair)
        .count();
    let sol2 = "⭐ Merry Christmas! ⭐";

    (Solution::from(sol1), Solution::from(sol2))
}

fn check_pair((lock, key): &(Heights, Heights)) -> bool {
    lock.iter().zip(key.iter()).all(|(l, k)| l + k < 6)
}

fn parse(input: &str) -> (Vec<Heights>, Vec<Heights>) {
    let (mut locks, mut keys) = (vec![], vec![]);
    for schema in input.split(DOUBLE_NEWLINE) {
        let grid = Matrix::from_str(schema);
        let heights: [u8; 5] = array::from_fn(|x| column_height(&grid, x));
        match grid[(0, 0)] {
            '#' => locks.push(heights),
             _  => keys.push(heights),
        }
    }

    (locks, keys)
}

fn column_height(grid: &Matrix<char>, x: usize) -> u8 {
    (0..7).filter(|&y| grid[(x, y)] == '#').count() as u8 - 1
}
