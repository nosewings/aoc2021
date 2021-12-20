use std::collections::HashMap;

use aoc2021::day_08::*;
use aoc2021::*;
use enumset::EnumSet;
use itertools::{iterate, Itertools};

fn solve(patterns: Patterns) -> HashMap<EnumSet<Segment>, u32> {
    let v = (&patterns
        .into_iter()
        .sorted_by_key(EnumSet::len)
        .group_by(EnumSet::len))
        .into_iter()
        .map(|(n, ps)| {
            let ps = ps.collect_vec();
            (n, ps.len(), ps)
        })
        .collect_vec();
    let (one, seven, four, two_three_five, zero_six_nine, eight) = match &v[..] {
        [(2, 1, one), (3, 1, seven), (4, 1, four), (5, 3, two_three_five), (6, 3, zero_six_nine), (7, 1, eight)] => {
            (one, seven, four, two_three_five, zero_six_nine, eight)
        }
        _ => panic!(),
    };
    let one = one[0];
    let seven = seven[0];
    let four = four[0];
    let eight = eight[0];
    let (six, zero_nine) = zero_six_nine
        .iter()
        .partition::<Vec<EnumSet<Segment>>, _>(|p| !p.is_superset(one));
    let six = six[0];
    let (nine, zero) = zero_nine
        .iter()
        .partition::<Vec<EnumSet<Segment>>, _>(|p| p.is_superset(four));
    let nine = nine[0];
    let zero = zero[0];
    let (three, two_five) = two_three_five
        .iter()
        .partition::<Vec<EnumSet<Segment>>, _>(|p| p.is_superset(one));
    let three = three[0];
    let (five, two) = two_five
        .iter()
        .partition::<Vec<EnumSet<Segment>>, _>(|p| p.is_subset(six));
    let five = five[0];
    let two = two[0];
    [
        (zero, 0),
        (one, 1),
        (two, 2),
        (three, 3),
        (four, 4),
        (five, 5),
        (six, 6),
        (seven, 7),
        (eight, 8),
        (nine, 9),
    ]
    .into_iter()
    .collect()
}

fn run(input: Vec<Displays>) -> u32 {
    input.into_iter().fold_map(|displays| {
        let solution = solve(displays.patterns);
        displays
            .outputs
            .iter()
            .map(|output| solution.get(output).unwrap())
            .rev()
            .zip(iterate(1, |n| 10 * n))
            .map(|(x, y)| x * y)
            .sum()
    })
}

make_main! {8, parse_input, run}
make_test! {08, 2, parse_input, run, 1073431}
