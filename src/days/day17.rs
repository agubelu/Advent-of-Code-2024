use std::fs::read_to_string;
use itertools::Itertools;

use crate::etc::DOUBLE_NEWLINE;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type Reg = u64;
struct VM<'a> {
    ip: usize,
    a: Reg,
    b: Reg,
    c: Reg,
    program: &'a[u8]
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day17.txt").unwrap();
    let (a, _, _, program) = parse(&input);

    let sol1 = VM::output(a, &program).iter().join(",");
    let sol2 = find_smallest_argument(&program);

    (Solution::from(sol1), Solution::from(sol2))
}

fn find_smallest_argument(program: &[u8]) -> u64 {
    // The n-th position in the output changes every 8^n input values
    // So, a program of length n will output itself for an input in the range [8^(n-1), 8^n)
    let len = program.len() as u32;
    let start = 8u64.pow(len - 1);
    let end = 8u64.pow(len);
    find_position(len as usize - 1, program, start, end).unwrap()
}

fn find_position(pos: usize, program: &[u8], mut start: u64, end: u64) -> Option<u64> {
    let block_size = 8u64.pow(pos as u32);
    while start < end {
        // We only need to try the first input each block, because the output for the desired position
        // will be the same for all values in the block.
        // Using .nth() allows us to only run the VM up to the desired output position, saving some time.
        let mut output = VM::new(start, 0, 0, program);
        if output.nth(pos).unwrap() == program[pos] {
            if pos == 0 {
                // First position has block size = 1, if it matches, it's the desired input.
                return Some(start);
            }
            // Propagate upwards when the desired input has been found.
            if let Some(input) = find_position(pos - 1, program, start, start + block_size) {
                return Some(input);
            }
        }
        start += block_size;
    }

    None
}

fn parse(input: &str) -> (Reg, Reg, Reg, Vec<u8>) {
    let (regs, prog) = input.split_once(DOUBLE_NEWLINE).unwrap();
    let (a, b, c) = regs.lines().map(
        |line| line.split_once(": ").unwrap().1.parse().unwrap()
    ).collect_tuple().unwrap();

    let program = prog.split_once(": ").unwrap().1.trim()
        .split(',').map(|x| x.parse().unwrap()).collect();
    (a, b, c, program)
}

///////////////////////////////////////////////////////////////////////////////

macro_rules! adv {
    ($self:ident, $arg:expr, $target:tt) => {
        $self.$target = $self.a / (2 as Reg).pow($self.combo_val($arg) as u32)
    };
}

impl<'a> VM<'a> {
    pub fn output(a_init: Reg, program: &'a[u8]) -> Vec<u8> {
        VM::new(a_init, 0, 0, program).collect()
    }

    pub fn new(a: Reg, b: Reg, c: Reg, program: &'a[u8]) -> Self {
        Self { a, b, c, program, ip: 0 }
    }

    #[allow(clippy::assign_op_pattern)] // Clippy isn't smart enough to figure out the macro abstraction
    pub fn run(&mut self) -> Option<u8> {
        // Runs the VM until a value is returned or execution stops
        while self.ip < self.program.len() {
            let (opcode, arg) = (self.program[self.ip], self.program[self.ip + 1]);
            match opcode {
                0 => adv!(self, arg, a),
                1 => self.b ^= arg as Reg,
                2 => self.b = self.combo_val(arg) & 7,
                3 => {
                    if self.a != 0 {
                        self.ip = arg as usize;
                        continue; // Prevent incrementing IP after jmp
                    }
                },
                4 => self.b ^= self.c,
                5 => {
                    self.ip += 2;  // Increment IP before returning
                    return Some((self.combo_val(arg) & 7) as u8);
                },
                6 => adv!(self, arg, b),
                7 => adv!(self, arg, c),
                _ => unreachable!(),
            };
            self.ip += 2;
        }

        None
    }

    fn combo_val(&self, val: u8) -> Reg {
        match val {
            0..=3 => val as Reg,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
}

impl<'a> Iterator for VM<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.run()
    }
}
