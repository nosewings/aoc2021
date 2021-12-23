use aoc2021::day_15::*;
use aoc2021::*;
use ndarray::Array2;

fn run(input: Array2<u32>) -> u32 {
    dijkstra(input)
}

make_main! {15, parse_input, run}
