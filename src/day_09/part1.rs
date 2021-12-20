use aoc2021::day_09::*;
use aoc2021::*;
use ndarray::Array2;

fn run(input: Array2<u32>) -> u32 {
    lows(&input).fold_map(|p| input[p] + 1)
}

make_main! {9, parse_input, run}
make_test! {09, 1, parse_input, run, 502}
