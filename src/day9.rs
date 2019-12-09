use std::iter;
use aoc_runner_derive::aoc;

use crate::intcode;

fn run_boost(mut program: Vec<i64>, input: i64) -> i64 {
    let mut output = Vec::new();
    intcode::execute(
        &mut program,
        iter::once(input),
        |value| output.push(value),
    );

    assert_eq!(output.len(), 1);

    output[0]
}

#[aoc(day9, part1)]
fn part1(input: &str) -> i64 {
    run_boost(intcode::parse_program(input), 1)
}

#[aoc(day9, part2)]
fn part2(input: &str) -> i64 {
    run_boost(intcode::parse_program(input), 2)
}
