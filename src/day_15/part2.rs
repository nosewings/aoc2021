use aoc2021::day_15::*;
use aoc2021::*;
use itertools::Itertools;
use ndarray::{s, Array2};

fn run(input: Array2<u32>) -> u32 {
    let (h, w) = input.shape2();
    let mut data = Array2::zeros((h * 5, w * 5));
    for (y, x) in (0..5).cartesian_product(0..5) {
        let mut view = data.slice_mut(s![y * h..(y + 1) * h, x * h..(x + 1) * h]);
        for (ix, n) in view.indexed_iter_mut() {
            *n = input[ix] + (y + x) as u32;
            if *n > 9 {
                *n -= 9;
            }
        }
    }
    dijkstra(data)
}

make_main! {15, parse_input, run}
make_test! {15, 2, parse_input, run, 2872}
