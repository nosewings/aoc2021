use aoc2021::day_02::*;
use aoc2021::*;

fn run(input: Vec<Command>) -> i32 {
    let (x, y): (i32, i32) = input.iter().fold_map(|command| match command.direction {
        Direction::Forward => (command.magnitude, 0),
        Direction::Down => (0, command.magnitude),
        Direction::Up => (0, -command.magnitude),
    });
    x * y
}

make_main! {2, parse_command, run}
make_test! {02, 1, parse_command, run, 2039256}
