use std::convert::TryInto;
use std::iter;
use aoc_runner_derive::aoc;

fn execute(mut program: &mut [usize], noun: usize, verb: usize) -> usize {
    program[1] = noun;
    program[2] = verb;

    for index in iter::successors(Some(0), |n| Some(n + 4)) {
        let op = program[index];
        match op {
            1 => operate(&mut program, index, |a, b| a + b),
            2 => operate(&mut program, index, |a, b| a * b),
            99 => break,
            _ => panic!("erroneous program!"),
        }
    }

    program[0]
}

fn operate<F>(program: &mut [usize], index: usize, f: F)
where
    F: FnOnce(usize, usize) -> usize,
{
    let arg: [usize; 3] = program[index+1..index+4].try_into().unwrap();
    program[arg[2]] = f(program[arg[0]], program[arg[1]]);
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    let mut program = parse_input(input);
    execute(&mut program, 12, 2)
}

#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
    let program = parse_input(input);
    let mut state = program.clone();

    for noun in 0..=99 {
        for verb in 0..=99 {
            if execute(&mut state, noun, verb) == 19690720 {
                return 100 * noun + verb;
            }
            state.copy_from_slice(&program);
        }
    }

    panic!("no noun/verb match");
}
