use aoc2021::day_18::*;
use itertools::Itertools;

fn run(input: Vec<Number>) -> u32 {
    input
        .into_iter()
        .fold1(std::ops::Add::add)
        .unwrap()
        .magnitude()
}

aoc2021::make_main! {18, parse_input, run}
aoc2021::make_test! {18, 1, parse_input, run, 3494}
