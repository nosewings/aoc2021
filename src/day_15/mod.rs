use std::collections::BinaryHeap;

use ndarray::Array2;
use nom::character::complete::{digit1, newline};
use nom::combinator::{all_consuming, map_res};
use nom::multi::separated_list1;
use nom::{IResult, InputIter};

use crate::{newline_terminated, Array2Ext};

pub fn parse_input<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Array2<u32>> {
    map_res(
        all_consuming(newline_terminated(separated_list1(
            newline,
            map_res(digit1, |s: &'a str| {
                s.iter_elements()
                    .map(|c| str::parse(&c.to_string()))
                    .collect::<Result<Vec<_>, _>>()
            }),
        ))),
        Array2::from_rows,
    )
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Path {
    ix: (usize, usize),
    risk: u32,
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // BinaryHeap is a max-heap, and we always want to find the
        // least expensive path, so we reverse the ordering.
        other.risk.cmp(&self.risk)
    }
}

pub fn dijkstra(input: Array2<u32>) -> u32 {
    let (h, w) = input.shape2();
    let target = (h - 1, w - 1);
    let mut heap = BinaryHeap::new();
    heap.push(Path {
        ix: (0, 0),
        risk: 0,
    });
    let mut risks = Array2::from_elem(input.shape2(), u32::MAX);
    loop {
        let next = heap.pop().unwrap();
        if next.ix == target {
            break next.risk;
        }
        if next.risk > risks[next.ix] {
            continue;
        }
        for ix in input.cardinal_neighbor_indices(next.ix).into_iter() {
            let risk = next.risk + input[ix];
            if risk < risks[ix] {
                heap.push(Path { ix, risk });
                risks[ix] = risk;
            }
        }
    }
}
