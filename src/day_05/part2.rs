use std::collections::HashMap;

use aoc2021::day_05::*;
use aoc2021::*;

fn run(input: Vec<Vents>) -> usize {
    let mut heights = HashMap::new();
    for vents in input {
        for point in vents.covered() {
            heights.entry(point).and_modify(|h| *h += 1).or_insert(1);
        }
    }
    heights.iter().filter(|(_, n)| **n >= 2).count()
}

make_main! {5, parse_vents, run}
make_test! {05, 2, parse_vents, run, 18627}
