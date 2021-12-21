use aoc2021::day_10::*;
use aoc2021::*;
use itertools::Itertools;
use BracketType::*;
use Outcome::*;

fn score(line: &[Character]) -> Option<u64> {
    match scan(line) {
        Corrupted(_) => None,
        Incomplete(stack) => Some(stack.iter().rev().fold(0, |acc, c| {
            5 * acc
                + match c {
                    Paren => 1,
                    Bracket => 2,
                    Brace => 3,
                    Angle => 4,
                }
        })),
    }
}

fn run(input: Vec<Vec<Character>>) -> u64 {
    let mut result = input.iter().flat_map(|v| score(v)).collect_vec();
    result.sort_unstable();
    result[result.len() / 2]
}

make_main! {10, parse_input, run}
make_test! {10, 2, parse_input, run, 1118645287}
