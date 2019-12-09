use std::iter;

#[derive(Clone, Copy)]
enum IntOp {
    Add, Mul, Lt, Eq,
}

pub fn execute<I, O>(
    program: &mut Vec<i64>,
    mut input: I,
    mut output: O,
)
where
    I: Iterator<Item = i64>,
    O: FnMut(i64),
{
    let mut index = 0;
    let mut base = 0;
    loop {
        let opcode = program[index] as usize;
        index = match opcode % 100 {
            1 => int_op(program, index, base, opcode, IntOp::Add),
            2 => int_op(program, index, base, opcode, IntOp::Mul),
            7 => int_op(program, index, base, opcode, IntOp::Lt),
            8 => int_op(program, index, base, opcode, IntOp::Eq),
            5 => jmp(program, index, base, opcode, true),
            6 => jmp(program, index, base, opcode, false),
            3 => inp(program, index, base, opcode, &mut input),
            4 => out(program, index, base, opcode, &mut output),
            9 => rel(program, index, opcode, &mut base),
            99 => break,
            _ => panic!("invalid opcode {}", opcode),
        };
    }
}

fn extract_params(
    program: &mut Vec<i64>,
    params: &mut [i64],
    write_start: usize,
    base: i64,
    opcode: usize,
) {
    let mut modes = iter::successors(
            Some(opcode / 100),
            |n| Some(n / 10)
        )
        .map(|n| n % 10);
    
    params[..write_start]
        .iter_mut()
        .zip(&mut modes)
        .for_each(|(param, mode)| match mode {
            0 => *param = program[*param as usize],
            2 => *param = program[(*param + base) as usize],
            _ => (),
        });
    
    params[write_start..]
        .iter_mut()
        .zip(modes)
        .for_each(|(param, mode)| if mode == 2 {
            *param += base;
        })
}

fn int_op(
    program: &mut Vec<i64>,
    index: usize,
    base: i64,
    opcode: usize,
    op: IntOp,
) -> usize {
    let mut par = [program[index + 1], program[index + 2], program[index + 3]];
    extract_params(program, &mut par, 2, base, opcode);

    let out = &mut program[par[2] as usize];

    match op {
        IntOp::Add => *out = par[0] + par[1],
        IntOp::Mul => *out = par[0] * par[1],
        IntOp::Lt => *out = (par[0] < par[1]) as i64,
        IntOp::Eq => *out = (par[0] == par[1]) as i64,
    }

    index + 4
}

fn jmp(
    program: &mut Vec<i64>,
    index: usize,
    base: i64,
    opcode: usize,
    mode: bool,
) -> usize {
    let mut par = [program[index + 1], program[index + 2]];
    extract_params(program, &mut par, 2, base, opcode);

    if mode ^ (par[0] == 0) {
        par[1] as usize
    } else {
        index + 3
    }
}

fn out<O>(
    program: &mut Vec<i64>,
    index: usize,
    base: i64,
    opcode: usize,
    mut output: O,
) -> usize
where
    O: FnMut(i64),
{
    let mut par = [program[index + 1]];
    extract_params(program, &mut par, 1, base, opcode);

    output(par[0]);

    index + 2
}

fn inp<I>(
    program: &mut Vec<i64>,
    index: usize,
    base: i64,
    opcode: usize,
    mut input: I,
) -> usize
where
    I: Iterator<Item = i64>,
{
    let mut par = [program[index + 1]];
    extract_params(program, &mut par, 0, base, opcode);

    program[par[0] as usize] = input.next().unwrap();

    index + 2
}

fn rel(
    program: &mut Vec<i64>,
    index: usize,
    opcode: usize,
    base: &mut i64,
) -> usize {
    let mut par = [program[index + 1]];
    extract_params(program, &mut par, 1, *base, opcode);

    *base += par[0];

    index + 2
}

pub fn parse_program(input: &str) -> Vec<i64> {
    let mut program = vec![0; 10000];

    input
        .trim()
        .split(",")
        .map(|num| num.parse().unwrap())
        .zip(program.iter_mut())
        .for_each(|(num, target)| *target = num);

    program
}
