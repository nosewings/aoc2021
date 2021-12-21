use aoc2021::day_09::*;
use aoc2021::*;
use itertools::Itertools;
use ndarray::Array2;

fn explore(
    input: &Array2<u32>,
    flags: &mut Array2<bool>,
    ix: (usize, usize),
    prev: Option<u32>,
) -> usize {
    let here = input[ix];
    if flags[ix] || here == 9 || prev.map_or(false, |n| here < n) {
        return 0;
    }
    flags[ix] = true;
    1 + input
        .cardinal_neighbor_indices(ix)
        .into_iter()
        .fold_map(|ix| explore(input, flags, ix, Some(here)))
}

fn run(input: Array2<u32>) -> usize {
    let mut flags = Array2::from_elem(input.shape2(), false);
    lows(&input)
        .map(|ix| explore(&input, &mut flags, ix, None))
        .sorted()
        .rev()
        .take(3)
        .product()
}

make_main! {9, parse_input, run}
make_test! {09, 2, parse_input, run, 1330560}
