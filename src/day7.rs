use std::iter;
use std::thread;
use std::sync::mpsc;
use aoc_runner_derive::aoc;

use crate::intcode;

fn phase_sequence() -> impl Iterator<Item = [u8; 5]> {
    iter::successors(
            Some([0; 5]),
            |&[a, b, c, d, e]| {
                if e == 4 {
                    if d == 4 {
                        if c == 4 {
                            if b == 4 {
                                if a == 4 {
                                    None
                                } else {
                                    Some([a + 1, 0, 0, 0, 0])
                                }
                            } else {
                                Some([a, b + 1, 0, 0, 0])
                            }
                        } else {
                            Some([a, b, c + 1, 0, 0])
                        }
                    } else {
                        Some([a, b, c, d + 1, 0])
                    }
                } else {
                    Some([a, b, c, d, e + 1])
                }
            },
        )
        .filter(|&seq| {
            let mut seq = Vec::from(&seq[..]);
            seq.sort_unstable();
            seq.dedup();
            seq.len() == 5
        })
}

fn amplify_signal(program: &mut [i32], phase: u8, signal: i32) -> i32 {
    let mut output = 0;
    intcode::execute(
        program,
        [phase as i32, signal].iter().copied(),
        |value| output = value,
    );
    output
}

fn start_feedback(mut program: Vec<i32>, phase: u8)
    -> mpsc::SyncSender<(mpsc::SyncSender<i32>, mpsc::Receiver<i32>)>
{
    let (tx, rx) = mpsc::sync_channel(0);
    thread::spawn(move || {
        let (tx, rx): (mpsc::SyncSender<i32>, mpsc::Receiver<i32>) = rx.recv().unwrap();
        intcode::execute(
            &mut program,
            iter::once(phase as i32)
                .chain(rx.iter()),
            move |value| tx.send(value).unwrap(),
        );
    });
    tx
}

#[aoc(day7, part1)]
fn part1(input: &str) -> i32 {
    let program = intcode::parse_program(input);
    let mut state = vec![0; program.len()];

    phase_sequence()
        .map(|seq| {
            seq
                .iter()
                .fold(0, |signal, &phase| {
                    state.copy_from_slice(&program);
                    amplify_signal(&mut state, phase, signal)
                })
        })
        .max()
        .unwrap()
}

#[aoc(day7, part2)]
fn part2(input: &str) -> i32 {
    let program = intcode::parse_program(input);

    phase_sequence()
        .map(|seq| {
            let (tx, r1) = mpsc::sync_channel(0);
            let (t1, r2) = mpsc::sync_channel(0);
            let (t2, r3) = mpsc::sync_channel(0);
            let (t3, r4) = mpsc::sync_channel(0);
            let (t4, r5) = mpsc::sync_channel(0);
            let (t5, rx) = mpsc::sync_channel(0);

            let mut init = seq
                .iter()
                .map(|&phase| start_feedback(program.clone(), phase + 5));
            
            init.next().unwrap().send((t1, r1)).unwrap();
            init.next().unwrap().send((t2, r2)).unwrap();
            init.next().unwrap().send((t3, r3)).unwrap();
            init.next().unwrap().send((t4, r4)).unwrap();
            {init}.next().unwrap().send((t5, r5)).unwrap();

            iter::successors(
                    Some(0),
                    move |&signal| tx
                        .send(signal)
                        .ok()
                        .and_then(|_| rx.recv().ok()),
                )
                .last()
                .unwrap()
        })
        .max()
        .unwrap()
}
