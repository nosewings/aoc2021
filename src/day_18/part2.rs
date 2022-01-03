use aoc2021::day_18::*;
use itertools::Itertools;

fn run(input: Vec<Number>) -> u32 {
    let other = input.clone();
    input
        .into_iter()
        .enumerate()
        .cartesian_product(other.into_iter().enumerate())
        .filter_map(|((i, x), (j, y))| {
            if i != j {
                Some((x + y).magnitude())
            } else {
                None
            }
        })
        .max()
        .unwrap()
}

aoc2021::make_main! {18, parse_input, run}
aoc2021::make_test! {18, 2, parse_input, run, 4712}
