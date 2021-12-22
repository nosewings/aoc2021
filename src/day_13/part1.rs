use aoc2021::day_13::*;
use aoc2021::*;

fn run(mut input: Input) -> usize {
    input.folds[0].execute(&mut input.dots);
    input.dots.len()
}

make_main! {13, parse_input, run}
make_test! {13, 1, parse_input, run, 671}
