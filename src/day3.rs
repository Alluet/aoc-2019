use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use aoc_runner_derive::aoc;

type Movement = (i64, i64, i64);

fn parse_distance(distance: &str) -> i64 {
    distance.parse().unwrap()
}

fn parse_input<'a>(input: &'a str) -> [impl Iterator<Item = Movement> + 'a; 2] {
    let mut iter = input
        .lines()
        .map(|line| {
            line
                .split(",")
                .map(|movement| {
                    match &movement[0..1] {
                        "R" => (1, 0, parse_distance(&movement[1..])),
                        "U" => (0, 1, parse_distance(&movement[1..])),
                        "D" => (0, -1, parse_distance(&movement[1..])),
                        "L" => (-1, 0, parse_distance(&movement[1..])),
                        _ => panic!("invalid movement"),
                    }
                })
        });
    
    [iter.next().unwrap(), iter.next().unwrap()]
}

#[aoc(day3, part1)]
fn part1(input: &str) -> i64 {
    let [first, second] = parse_input(input);

    let pos = RefCell::new((0, 0));
    let pos_ref = &pos;
    let wiring: HashSet<(i64, i64)> = first
        .flat_map(|(dx, dy, distance)| {
            (0..distance)
                .map(move |_| {
                    let mut pos = pos_ref.borrow_mut();
                    pos.0 += dx;
                    pos.1 += dy;
                    *pos
                })
        })
        .collect();

    pos.replace((0, 0));
    second
        .flat_map(|(dx, dy, distance)| {
            (0..distance)
                .map(move |_| {
                    let mut pos = pos_ref.borrow_mut();
                    pos.0 += dx;
                    pos.1 += dy;
                    pos
                })
        })
        .filter_map(|pos| {
            if wiring.contains(&pos) {
                Some(pos.0.abs() + pos.1.abs())
            } else {
                None
            }
        })
        .min().unwrap()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> i64 {
    let [first, second] = parse_input(input);

    let pos = RefCell::new((0, 0));
    let pos_ref = &pos;
    let wiring: HashMap<(i64, i64), i64> = first
        .flat_map(|(dx, dy, distance)| {
            (0..distance)
                .map(move |_| {
                    let mut pos = pos_ref.borrow_mut();
                    pos.0 += dx;
                    pos.1 += dy;
                    *pos
                })
        })
        .zip(1..)
        .collect();

    pos.replace((0, 0));
    second
        .flat_map(|(dx, dy, distance)| {
            (0..distance)
                .map(move |_| {
                    let mut pos = pos_ref.borrow_mut();
                    pos.0 += dx;
                    pos.1 += dy;
                    pos
                })
        })
        .zip(1..)
        .filter_map(|(pos, distance2)| {
            if let Some(&distance1) = wiring.get(&pos) {
                Some(distance1 + distance2)
            } else {
                None
            }
        })
        .min().unwrap()
}
