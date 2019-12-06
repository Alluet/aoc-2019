use std::iter;
use std::collections::HashMap;
use aoc_runner_derive::aoc;

fn parse_input<'a>(input: &'a str) -> HashMap<&'a str, &'a str> {
    input
        .lines()
        .map(|n| {
            let mut iter = n.split(")");
            let inner = iter.next().unwrap();
            (iter.next().unwrap(), inner)
        })
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    let orbits = parse_input(input);

    orbits.iter()
        .flat_map(
            |(&outer, &inner)| iter::successors(
                Some((outer, inner)),
                |(_, outer)| {
                    orbits
                        .get(outer)
                        .map(|&inner| {
                            (*outer, inner)
                        })
                },
            )
        )
        .count()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> u64 {
    let orbits = parse_input(input);

    let make_chain = |start| iter::successors(
            Some(("", start)),
            |(_, outer)| {
                orbits
                    .get(outer)
                    .map(|&inner| {
                        (*outer, inner)
                    })
            },
        )
        .skip(1)
        .zip(0..);

    let you_chain: HashMap<&str, u64> = make_chain("YOU")
        .map(|((_, inner), distance)| (inner, distance))
        .collect();
    
    let (intersection, distance) = make_chain("SAN")
        .take_while(|((outer, _), _)| !you_chain.contains_key(outer))
        .map(|((_, inner), distance)| (inner, distance))
        .last()
        .unwrap();
    
    distance + you_chain[intersection]
}
