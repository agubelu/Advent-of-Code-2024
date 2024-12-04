use std::fs::read_to_string;
use crate::etc::{Coords2D, Matrix};
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type Pos = Coords2D<i32>;

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day04.txt").unwrap();
    let matrix = Matrix::from_str(&input);

    let (mut sol1, mut sol2) = (0, 0);
    let dirs = Pos::origin().neighbors_diag();

    for (pos, ch) in matrix.enumerate() {
        match ch {
            'X' => sol1 += find_xmas(&matrix, pos, &dirs),
            'A' => sol2 += find_cross_mas(&matrix, pos),
             _  => continue
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}

fn find_xmas(matrix: &Matrix<char>, pos: Pos, dirs: &[Pos]) -> usize {
    dirs.iter().copied()
        .filter(|dir| matrix.get_or_dot(pos + dir) == 'M' &&
                      matrix.get_or_dot(pos + dir * 2) == 'A' &&
                      matrix.get_or_dot(pos + dir * 3) == 'S')
        .count()
}

fn find_cross_mas(matrix: &Matrix<char>, pos: Pos) -> usize {
    let d1 = (matrix.get_or_dot(pos.up().left()), matrix.get_or_dot(pos.down().right()));
    let d2 = (matrix.get_or_dot(pos.up().right()), matrix.get_or_dot(pos.down().left()));
    [d1, d2].into_iter().all(|d| matches!(d, ('M', 'S') | ('S', 'M'))) as usize
}
