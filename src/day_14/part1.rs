use std::collections::HashMap;

use aoc2021::day_14::*;
use aoc2021::*;
use itertools::Itertools;

struct State {
    rules: HashMap<(char, char), char>,
    polymer: Vec<char>,
    buf: Vec<char>,
}

impl From<Input> for State {
    fn from(input: Input) -> Self {
        State {
            rules: input.rules,
            polymer: input.template,
            buf: Vec::new(),
        }
    }
}

impl State {
    fn step(&mut self) {
        self.buf.clear();
        self.buf.reserve(2 * self.polymer.len());
        self.polymer
            .iter()
            .map(Some)
            .interleave(
                self.polymer
                    .iter()
                    .copied()
                    .tuple_windows()
                    .map(|p| self.rules.get(&p)),
            )
            .flatten()
            .for_each(|c| self.buf.push(*c));
        std::mem::swap(&mut self.polymer, &mut self.buf);
    }
}

fn run(input: Input) -> u32 {
    let mut state = State::from(input);
    for _ in 1..=10 {
        state.step();
    }
    let mut spectrum = HashMap::new();
    for c in state.polymer {
        *spectrum.entry(c).or_insert(0) += 1;
    }
    let results = spectrum.into_iter().map(|p| p.1).sorted().collect_vec();
    results.last().unwrap() - results.first().unwrap()
}

make_main! {14, parse_input, run}
