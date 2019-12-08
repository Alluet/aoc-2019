use std::fmt;
use std::str;
use aoc_runner_derive::aoc;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .as_bytes()
        .chunks(25*6)
        .map(|bytes| bytes.into())
        .collect()
}

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let layers = parse_input(input);

    layers
        .into_iter()
        .map(|layer| {
            let count = move |n| {
                layer
                    .iter()
                    .filter(|&&c| c == n)
                    .count()
            };

            (count(b'0'), count(b'1') * count(b'2'))
        })
        .min_by_key(|&(zeroes, _)| zeroes)
        .unwrap()
        .1
}

#[aoc(day8, part2)]
fn part2(input: &str) -> Part2 {
    let layers = parse_input(input);

    layers
        .into_iter()
        .rev()
        .flat_map(|layer| {
            layer
                .into_iter()
                .enumerate()
        })
        .fold(
            Part2(vec![' '; 25*6]),
            |Part2(mut image), (i, color)| {
                match color {
                    b'0' => image[i] = '⬛',
                    b'1' => image[i] = '⬜',
                    _ => (),
                }
                Part2(image)
            }
        )
}

struct Part2(Vec<char>);

impl fmt::Display for Part2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.0.chunks(25) {
            write!(f, "\n{}", line.iter().collect::<String>())?;
        }

        Ok(())
    }
}
