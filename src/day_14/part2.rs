use std::collections::HashMap;

use aoc2021::day_14::*;
use aoc2021::*;
use counter::Counter;
use itertools::Itertools;

struct State {
    count: Counter<char>,
    pairs: Counter<(char, char)>,
    rules: HashMap<(char, char), char>,
}

impl From<Input> for State {
    fn from(input: Input) -> Self {
        State {
            count: input.template.iter().copied().collect(),
            pairs: input
                .template
                .iter()
                .copied()
                .tuple_windows::<(char, char)>()
                .collect(),
            rules: input.rules,
        }
    }
}

impl State {
    fn step(&mut self) {
        let mut new_pairs = self.pairs.clone();
        for (p @ (a, b), &x) in self.rules.iter() {
            let n = self.pairs[p];
            self.count[&x] += n;
            new_pairs[p] -= n;
            new_pairs[&(*a, x)] += n;
            new_pairs[&(x, *b)] += n;
        }
        self.pairs = new_pairs;
    }
}

fn run(input: Input) -> usize {
    let mut state = State::from(input);
    for _ in 1..=40 {
        state.step();
    }
    let results = state.count.most_common();
    results.first().unwrap().1 - results.last().unwrap().1
}

make_main! {14, parse_input, run}
make_test! {14, 2, parse_input, run, 2158894777814}
