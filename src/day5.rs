use std::iter;
use aoc_runner_derive::aoc;

#[derive(Clone, Copy)]
enum IntOp {
    Add, Mul, Lt, Eq,
}

fn execute<I, O>(
    program: &mut [i32],
    mut input: I,
    mut output: O,
)
where
    I: Iterator<Item = i32>,
    O: FnMut(i32),
{
    let mut index = 0;
    loop {
        let opcode = program[index] as usize;

        index = match opcode % 100 {
            1 => int_op(program, index, opcode, IntOp::Add),
            2 => int_op(program, index, opcode, IntOp::Mul),
            7 => int_op(program, index, opcode, IntOp::Lt),
            8 => int_op(program, index, opcode, IntOp::Eq),
            5 => jmp(program, index, opcode, true),
            6 => jmp(program, index, opcode, false),
            4 => out(program, index, opcode, &mut output),
            3 => inp(program, index, &mut input),
            99 => break,
            _ => panic!("invalid opcode"),
        };
    }
}

fn extract_params(program: &mut [i32], params: &mut [i32], opcode: usize) {
    for (param, position) in params
        .iter_mut()
        .zip(
            iter::successors(Some(opcode / 100), |n| Some(n / 10))
                .map(|n| n % 2 == 0)
        )
    {
        if position {
            *param = program[*param as usize];
        }
    }
}

fn int_op(
    program: &mut [i32],
    index: usize,
    opcode: usize,
    op: IntOp,
) -> usize {
    let mut par = [program[index + 1], program[index + 2]];
    extract_params(program, &mut par, opcode);

    let out = &mut program[program[index + 3] as usize];

    match op {
        IntOp::Add => *out = par[0] + par[1],
        IntOp::Mul => *out = par[0] * par[1],
        IntOp::Lt => *out = (par[0] < par[1]) as i32,
        IntOp::Eq => *out = (par[0] == par[1]) as i32,
    }

    index + 4
}

fn jmp(
    program: &mut [i32],
    index: usize,
    opcode: usize,
    mode: bool,
) -> usize {
    let mut par = [program[index + 1], program[index + 2]];
    extract_params(program, &mut par, opcode);

    if mode ^ (par[0] == 0) {
        par[1] as usize
    } else {
        index + 3
    }
}

fn out<O>(
    program: &mut [i32],
    index: usize,
    opcode: usize,
    mut output: O,
) -> usize
where
    O: FnMut(i32),
{
    let mut par = [program[index + 1]];
    extract_params(program, &mut par, opcode);

    output(par[0]);

    index + 2
}

fn inp<I>(
    program: &mut [i32],
    index: usize,
    mut input: I,
) -> usize
where
    I: Iterator<Item = i32>,
{
    program[program[index + 1] as usize] = input.next().unwrap();

    index + 2
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect()
}

fn run_diagnostics(program: &mut [i32], id: i32) -> i32 {
    let mut output = Vec::new();
    execute(program, iter::once(id), |value| output.push(value));
    *output.last().unwrap()
}

#[aoc(day5, part1)]
fn part1(input: &str) -> i32 {
    run_diagnostics(&mut parse_input(input), 1)
}

#[aoc(day5, part2)]
fn part2(input: &str) -> i32 {
    run_diagnostics(&mut parse_input(input), 5)
}
