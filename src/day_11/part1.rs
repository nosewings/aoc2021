use aoc2021::day_11::*;
use aoc2021::*;
use ndarray::Array2;

fn run(mut input: Array2<u32>) -> u32 {
    (1..=100).fold_map(|_| step(&mut input))
}

make_main! {11, parse_input, run}
make_test! {11, 1, parse_input, run, 1617}
