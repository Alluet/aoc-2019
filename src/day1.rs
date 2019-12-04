use std::iter;
use std::num::NonZeroU64;
use aoc_runner_derive::aoc;

fn get_fuel(mass: u64) -> Option<u64> {
    (mass / 3)
        .checked_sub(2)
        .and_then(|n| NonZeroU64::new(n))
        .map(|n| n.get())
}

#[aoc(day1, part1)]
fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| get_fuel(line.parse().unwrap()).unwrap())
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let module: u64 = line.parse().unwrap();
            iter::successors(
                    get_fuel(module),
                    |&fuel| get_fuel(fuel),
                )
                .sum::<u64>()
        })
        .sum()
}
