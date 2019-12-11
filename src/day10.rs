use std::iter;
use std::ops::Range;
use std::collections::{HashSet, VecDeque};
use std::f64::consts::{PI, FRAC_PI_2};
use linked_hash_map::LinkedHashMap;
use num::Integer;
use aoc_runner_derive::aoc;

fn parse_input(input: &str) -> (HashSet<(i64, i64)>, Range<i64>, Range<i64>) {
    let mut max_x = 0;
    let mut max_y = 0;

    let set = input
        .lines()
        .zip(0..)
        .flat_map(|(line, y)| {
            line
                .bytes()
                .zip(0..)
                .filter_map(move |(byte, x)| {
                    if byte == b'#' {
                        Some((x, y))
                    } else { None }
                })
        })
        .inspect(|&(x, y)| {
            if x > max_x { max_x = x };
            if y > max_y { max_y = y };
        })
        .collect();
    
    (set, 0 .. max_x + 1, 0 .. max_y + 1)
}

fn sort_asteroids(
    asteroids: &HashSet<(i64, i64)>,
    (x, y): (i64, i64),
    x_range: Range<i64>,
    y_range: Range<i64>,
) -> Vec<((i64, i64), VecDeque<(i64, i64)>, f64)> {
    let mut checked: HashSet<_> = HashSet::new();

    asteroids
        .iter()
        .filter_map(|&(x2, y2)| {
            let (dx, dy) = (x2 - x, y2 - y);
            let div = dx.gcd(&dy);
            if div == 0 { return None };
            let (dx, dy) = (dx / div, dy / div);
            if checked.insert((dx, dy)) {
                let intersected =
                    iter::successors(
                        Some((x + dx, y + dy)),
                        |(x, y)| {
                            let (x, y) = (x + dx, y + dy);
                            if x_range.contains(&x) && y_range.contains(&y) {
                                Some((x, y))
                            } else { None }
                        }
                    )
                    .filter(|pos| asteroids.contains(pos))
                    .fold(VecDeque::new(), |mut intersected, pos| {
                        intersected.push_back(pos);
                        intersected
                    });
                let mut angle = (-dy as f64).atan2(dx as f64) - FRAC_PI_2;
                if angle > 0.0 {
                    angle = angle - 2.0*PI;
                }
                Some(((dx, dy), intersected, angle))
            } else { None }
        })
        .collect()
}

fn best_station(
    asteroids: &HashSet<(i64, i64)>,
    x_range: Range<i64>,
    y_range: Range<i64>,
) -> Vec<((i64, i64), VecDeque<(i64, i64)>, f64)> {
    asteroids
        .iter()
        .map(|&asteroid| {
            sort_asteroids(
                asteroids,
                asteroid,
                x_range.clone(),
                y_range.clone(),
            )
        })
        .max_by_key(|n| n.len())
        .unwrap()
}

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    let (asteroids, x_range, y_range) = parse_input(input);
    best_station(&asteroids, x_range, y_range).len()
}

#[aoc(day10, part2)]
fn part2(input: &str) -> i64 {
    let (asteroids, x_range, y_range) = parse_input(input);
    let mut asteroid_angles = best_station(&asteroids, x_range, y_range);
    asteroid_angles.sort_by(|(_, _, a), (_, _, b)| b.partial_cmp(a).unwrap());

    let mut by_ray: LinkedHashMap<_, _> = asteroid_angles
        .into_iter()
        .map(|(ray, asteroids, _)| (ray, asteroids))
        .collect();
    
    let mut count = 0;
    while !by_ray.is_empty() {
        for mut entry in by_ray.entries() {
            count += 1;

            let asteroids = entry.get_mut();
            let (x, y) = asteroids.pop_front().unwrap();
            if count == 200 {
                return x * 100 + y;
            } else if asteroids.is_empty() {
                let _ = entry.remove();
            }
        }
    }

    0
}
