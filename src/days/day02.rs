use std::fs::read_to_string;
use itertools::Itertools;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day02.txt").unwrap();
    let reports: Vec<Vec<i32>> = input.lines()
        .map(|line| line.split_whitespace().map(|x| x.parse().unwrap()).collect())
        .collect();

    let sol1 = reports.iter().filter(|r| is_safe_report(r)).count();
    let sol2 = reports.iter().filter(|r| is_safe_report_2(r)).count();

    (Solution::from(sol1), Solution::from(sol2))
}

fn is_safe_report(report: &[i32]) -> bool {
    let sign = (report[0] - report[1]).signum();
    report.iter().tuple_windows()
        .all(|(a, b)| (a - b) * sign > 0 && (a - b).abs() <= 3)
}

fn is_safe_report_2(report: &[i32]) -> bool {
    (0..report.len())
        .map(|ix| remove_elem(report, ix))
        .any(|r| is_safe_report(&r))
}

fn remove_elem(report: &[i32], ix: usize) -> Vec<i32> {
    let mut cloned = report.to_owned();
    cloned.remove(ix);
    cloned
}
