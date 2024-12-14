use std::fs::read_to_string;

use itertools::Itertools;
use sscanf::{sscanf, FromScanf};

use crate::etc::consts::DOUBLE_NEWLINE;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, Hash, FromScanf)]
#[sscanf(format_unescaped = r#"Button A: X\+{ax}, Y\+{ay}\nButton B: X\+{bx}, Y\+{by}\nPrize: X={tx}, Y={ty}"#)]
struct ClawMachine {
    ax: u64, ay: u64,
    bx: u64, by: u64,
    tx: u64, ty: u64,
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day13.txt").unwrap();
    let machines = input.trim().split(DOUBLE_NEWLINE).map(|s| sscanf!(s, "{ClawMachine}").unwrap()).collect_vec();

    let sol1: u64 = machines.iter().filter_map(|m| solve_machine(m, 0.0)).sum();
    let sol2: u64 = machines.iter().filter_map(|m| solve_machine(m, 1e13)).sum();

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_machine(machine: &ClawMachine, offset: f64) -> Option<u64> {
    // Solve the system of 2 equations
    let (ax, bx) = (machine.ax as f64, machine.bx as f64);
    let (ay, by) = (machine.ay as f64, machine.by as f64);
    let (tx, ty) = (machine.tx as f64 + offset, machine.ty as f64 + offset);

    let b_presses = (tx * ay - ax * ty) / (ay * bx - ax * by);
    let a_presses = (ty - by * b_presses) / ay;

    if a_presses.fract() == 0.0 && b_presses.fract() == 0.0 {
        Some((3.0 * a_presses + b_presses) as u64)
    } else {
        None
    }
}
