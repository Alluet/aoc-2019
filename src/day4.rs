use std::iter;
use aoc_runner_derive::aoc;

fn parse_input(input: &str) -> [u32; 2] {
    let mut iter = input
        .split("-")
        .map(|n| n.parse::<u32>().unwrap());
    
    [
        iter.next().unwrap(),
        iter.next().unwrap(),
    ]
}

fn get_digits(n: u32) -> [u8; 6] {
    let mut iter = iter::successors(
            Some(100000),
            |&n| Some(n / 10),
        )
        .map(|d| (n / d % 10) as u8);
    
    [
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ]
}

fn is_ordered(digits: [u8; 6]) -> bool {
    digits.windows(2)
        .filter(|window| window[1] < window[0])
        .next().is_none()
}

#[aoc(day4, part1)]
fn part1(input: &str) -> u32 {
    let [lower, upper] = parse_input(input);

    let mut matches = 0;
    for n in lower..=upper {
        let digits = get_digits(n);
        if is_ordered(digits) {
            if digits.windows(2)
                .filter(|window| window[0] == window[1])
                .next().is_some()
            {
                matches += 1;
            }
        }
    }

    matches
}

#[aoc(day4, part2)]
fn part2(input: &str) -> u32 {
    let [lower, upper] = parse_input(input);

    let mut matches = 0;
    for n in lower..=upper {
        let digits = get_digits(n);

        if is_ordered(digits) {
            let mut counts = [0u8; 10];
            for &digit in digits.iter() {
                counts[digit as usize] += 1;
            }
            for &count in counts.iter() {
                if count == 2 {
                    matches += 1;
                    break;
                }
            }
        }
    }

    matches
}
