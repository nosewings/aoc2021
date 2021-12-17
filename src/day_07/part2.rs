#![feature(int_abs_diff)]

use aoc2021::day_07::*;
use aoc2021::*;

fn error(input: &[u32], mean: u32) -> u32 {
    input.iter().fold_map(|n| {
        let d = mean.abs_diff(*n);
        d * (d + 1) / 2
    })
}

fn run(input: Vec<u32>) -> u32 {
    let mean = input.iter().sum::<u32>() as f32 / input.len() as f32;
    // Just rounding doesn't reliably work. Gotta try both.
    let a = mean.floor() as u32;
    let b = mean.ceil() as u32;
    error(&input, a).min(error(&input, b))
}

make_main! {7, parse_input, run}
make_test! {07, 2, parse_input, run, 95476244}
