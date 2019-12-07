use std::iter;
use aoc_runner_derive::aoc;

use crate::intcode;

fn run_diagnostics(program: &mut [i32], id: i32) -> i32 {
    let mut output = Vec::new();
    intcode::execute(program, iter::once(id), |value| output.push(value));
    *output.last().unwrap()
}

#[aoc(day5, part1)]
fn part1(input: &str) -> i32 {
    run_diagnostics(&mut intcode::parse_program(input), 1)
}

#[aoc(day5, part2)]
fn part2(input: &str) -> i32 {
    run_diagnostics(&mut intcode::parse_program(input), 5)
}
