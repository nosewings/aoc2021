use aoc2021::day_02::*;
use aoc2021::*;

fn run(input: Vec<Command>) -> i32 {
    let (x, y, _) = input
        .iter()
        .fold((0, 0, 0), |(x, y, aim), command| match command.direction {
            Direction::Down => (x, y, aim + command.magnitude),
            Direction::Up => (x, y, aim - command.magnitude),
            Direction::Forward => (x + command.magnitude, y + aim * command.magnitude, aim),
        });
    x * y
}

make_main! {2, parse_command, run}
make_test! {02, 2, parse_command, run, 1856459736}
