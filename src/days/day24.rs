use std::fs::read_to_string;
use itertools::Itertools;
use rustc_hash::FxHashMap;

use crate::etc::DOUBLE_NEWLINE;
use crate::{Solution, SolutionPair};

use BinaryGate::*;

///////////////////////////////////////////////////////////////////////////////

enum BinaryGate<'a> {
    Input(bool),
    Or(&'a str, &'a str),
    And(&'a str, &'a str),
    Xor(&'a str, &'a str),
}

#[derive(Default)]
struct Circuit<'a> {
    gates: FxHashMap<&'a str, BinaryGate<'a>>,
    reverse: FxHashMap<&'a str, Vec<&'a str>>,
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day24.txt").unwrap();
    let circuit = parse(&input);

    let sol1 = build_number(&circuit);
    let sol2 = find_wrong_wires(&circuit);

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn build_number(circuit: &Circuit) -> u64 {
    let zs = circuit.gates.keys().filter(|tag| tag.starts_with('z')).sorted().rev();
    let bits = zs.map(|tag| get_output(circuit, tag) as u64);
    bits.fold(0, |acc, i| (acc << 1) | i)
}

fn get_output(circuit: &Circuit, wire: &str) -> bool {
    match &circuit.gates[wire] {
        Input(b) => *b,
        Or(i1, i2) => get_output(circuit, i1) || get_output(circuit, i2),
        And(i1, i2) => get_output(circuit, i1) && get_output(circuit, i2),
        Xor(i1, i2) => get_output(circuit, i1) ^ get_output(circuit, i2),
    }
}

///////////////////////////////////////////////////////////////////////////////

fn find_wrong_wires(circuit: &Circuit) -> String {
    circuit.gates.keys()
        .filter(|wire| is_wrong_wire(circuit, wire))
        .sorted()
        .join(",")
}

fn is_wrong_wire(circuit: &Circuit, wire: &str) -> bool {
    let gate = &circuit.gates[wire];

    if wire.starts_with('z') && wire != "z45" && !matches!(gate, Xor(_, _)) {
        // Only XORs can provide output bits except for the highest one.
        return true;
    }

    match gate {
        Xor(i1, i2) => {
            if i1.starts_with(['x', 'y']) && i2.starts_with(['x', 'y']) {
                // Input XOR, it must output to another XOR and an AND unless it's the first bit
                if wire == "z00" { return false }
                let outs = &circuit.reverse[wire];
                !(outs.len() == 2 && outs.contains(&"AND") && outs.contains(&"XOR"))
            } else {
                // This must be an output XOR, make sure it actually outputs
                !wire.starts_with('z')
            }
        },
        And(i1, i2) if i1 != &"x00" && i2 != &"x00" => {
            // ANDs must output to exactly 1 OR
            let outs = &circuit.reverse[wire];
            outs.len() != 1 || outs[0] != "OR"
        },
        _ => false,
    }
}

fn parse(input: &str) -> Circuit {
    let mut circuit = Circuit::default();
    let (top, bottom) = input.split_once(DOUBLE_NEWLINE).unwrap();

    for line in top.lines() {
        let (tag, value) = line.split_once(": ").unwrap();
        circuit.gates.insert(tag, Input(value == "1"));
    }

    for line in bottom.lines() {
        let (i1, op, i2, _, tag) = line.split_whitespace().collect_tuple().unwrap();
        let gate = match op {
            "AND" => And(i1, i2),
            "OR" => Or(i1, i2),
            "XOR" => Xor(i1, i2),
            _ => unreachable!(),
        };
        circuit.gates.insert(tag, gate);
        circuit.reverse.entry(i1).or_default().push(op);
        circuit.reverse.entry(i2).or_default().push(op);
    }

    circuit
}
