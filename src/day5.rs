use std::iter;
use aoc_runner_derive::aoc;

use crate::intcode;

fn run_diagnostics(program: &mut Vec<i64>, id: i64) -> i64 {
    let mut output = Vec::new();
    intcode::execute(program, iter::once(id), |value| output.push(value));
    *output.last().unwrap()
}

#[aoc(day5, part1)]
fn part1(input: &str) -> i64 {
    run_diagnostics(&mut intcode::parse_program(input), 1)
}

#[aoc(day5, part2)]
fn part2(input: &str) -> i64 {
    run_diagnostics(&mut intcode::parse_program(input), 5)
}
