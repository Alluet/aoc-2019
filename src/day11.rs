use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::iter;
use std::fmt;
use aoc_runner_derive::aoc;

use crate::intcode;

#[derive(Clone, Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    fn right(&mut self) {
        *self = match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn left(&mut self) {
        *self = match self {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
        }
    }

    fn apply(self, pos: &mut (i64, i64)) {
        match self {
            Direction::Right => pos.0 += 1,
            Direction::Up => pos.1 -= 1,
            Direction::Left => pos.0 -= 1,
            Direction::Down => pos.1 += 1,
        }
    }
}

fn paint_panels(
    mut program: Vec<i64>,
    start_white: bool,
) -> HashMap<(i64, i64), bool> {
    let (input, rx) = mpsc::sync_channel(0);
    let (tx, output) = mpsc::sync_channel(0);

    thread::spawn(move || {
        intcode::execute(
            &mut program,
            rx.iter(),
            move |value| tx.send(value).unwrap(),
        );
    });

    let mut position = (0, 0);
    let mut direction = Direction::Up;
    let mut panels: HashMap<(i64, i64), bool> = HashMap::new();

    input.send(start_white as i64).unwrap();

    let output = iter::from_fn(move || {
        output
            .recv()
            .ok()
            .map(|white| {
                let white = white == 1;
                let right = output.recv().unwrap() == 1;
                (white, right)
            })
    });

    output
        .take_while(|&(white, right)| {
            panels.insert(position, white);

            if right { direction.right() }
            else { direction.left() };
            direction.apply(&mut position);

            let color = panels
                .get(&position)
                .map(|&n| n as i64)
                .unwrap_or(0);
            input.send(color).is_ok()
        })
        .for_each(|_| ());

    panels
}

#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    paint_panels(intcode::parse_program(input), false).len()
}

#[aoc(day11, part2)]
fn part2(input: &str) -> Part2 {
    let mut panels = paint_panels(intcode::parse_program(input), true);

    let (x1, y1, dx, dy) = {
        let &(mut x1, mut y1) = panels.keys().next().unwrap();
        let (mut x2, mut y2) = (x1, y1);
        panels.retain(|&(x, y), &mut white| {
            if white {
                if x < x1 { x1 = x; }
                else if x > x2 { x2 = x; }
                if y < y1 { y1 = y; }
                else if y > y2 { y2 = y; }
                true
            } else { false }
        });
        (x1, y1, (x2 - x1 + 1) as usize, (y2 - y1 + 1) as usize)
    };

    let mut canvas = vec![' '; dx*dy];
    panels
        .into_iter()
        .map(move |((x, y), _)| (((dx as i64) * (y - y1)) + (x - x1)) as usize)
        .for_each(|i| canvas[i] = '█');

    Part2(canvas, dx)
}

struct Part2(Vec<char>, usize);

impl fmt::Display for Part2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n┌─")?;
        for _ in 0..self.1 {
            write!(f, "─")?;
        }
        write!(f, "─┐")?;

        for line in self.0.chunks(self.1) {
            write!(f, "\n│ {} │", line.iter().collect::<String>())?;
        }

        write!(f, "\n└─")?;
        for _ in 0..self.1 {
            write!(f, "─")?;
        }
        write!(f, "─┘")?;

        Ok(())
    }
}
