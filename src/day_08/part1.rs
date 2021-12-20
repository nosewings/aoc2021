use aoc2021::day_08::*;
use aoc2021::*;

fn run(input: Vec<Displays>) -> usize {
    input
        .iter()
        .flat_map(|display| &display.outputs)
        .filter(|wires| [2, 3, 4, 7].contains(&wires.len()))
        .count()
}

make_main! {8, parse_input, run}
make_test! {08, 1, parse_input, run, 504}
