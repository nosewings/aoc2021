use aoc2021::day_11::*;
use aoc2021::*;
use ndarray::Array2;

fn run(mut input: Array2<u32>) -> u32 {
    let len = input.len() as u32;
    for i in 1.. {
        if step(&mut input) == len {
            return i;
        }
    }
    unreachable!();
}

make_main! {11, parse_input, run}
make_test! {11, 2, parse_input, run, 258}
