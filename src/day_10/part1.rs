use aoc2021::day_10::*;
use aoc2021::{FoldMap, *};
use BracketType::*;
use Outcome::*;

fn score(line: &[Character]) -> u32 {
    match scan(line) {
        Incomplete(_) => 0,
        Corrupted(c) => match c {
            Paren => 3,
            Bracket => 57,
            Brace => 1197,
            Angle => 25137,
        },
    }
}

fn run(input: Vec<Vec<Character>>) -> u32 {
    input.iter().fold_map(|v| score(v))
}

make_main! {10, parse_input, run}
make_test! {10, 2, parse_input, run, 366027}
