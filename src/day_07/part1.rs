use aoc2021::day_07::*;
use aoc2021::*;

fn run(mut input: Vec<u32>) -> u32 {
    input.sort_unstable();
    let mid = input.len() / 2;
    let median = if input.len() % 2 == 1 {
        input[mid]
    } else {
        let lo = input[mid - 1] as f32;
        let hi = input[mid] as f32;
        ((lo + hi) / 2_f32).round() as u32
    };
    input.iter().fold_map(|n| median.abs_diff(*n))
}

make_main! {7, parse_input, run}
make_test! {07, 1, parse_input, run, 339321}
