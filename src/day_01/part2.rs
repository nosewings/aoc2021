use aoc2021::day_01::*;
use aoc2021::*;
use itertools::Itertools;

pub fn run(input: Vec<u32>) -> u32 {
    input
        .iter()
        .tuple_windows()
        .map(|(x, y, z)| x + y + z)
        .tuple_windows()
        .fold_map(|(x, y)| (x < y) as u32)
}

make_main! {1, parse_input, run}
make_test! {01, 2, parse_input, run, 1362}
