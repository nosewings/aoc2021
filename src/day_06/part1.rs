use aoc2021::day_06::*;
use aoc2021::*;

fn run(input: Vec<u64>) -> u64 {
    run_for(input, 80)
}

make_main! {6, parse_input, run}
make_test! {06, 1, parse_input, run, 360610}
