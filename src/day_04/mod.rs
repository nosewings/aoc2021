use std::collections::HashMap;

use itertools::Itertools;
use nom::character::complete::{char, newline};
use nom::combinator::{all_consuming, map};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

use crate::*;

/// A `Layout` is a mapping from numbers to board positions.  This may
/// seem backwards, but it's a convenient representation for our
/// purposes.
pub type Layout = HashMap<usize, (usize, usize)>;

pub fn score_layout(layout: &Layout, numbers: &[usize]) -> usize {
    numbers.last().unwrap()
        * layout
            .keys()
            .filter(|n| !numbers.contains(n))
            .sum::<usize>()
}

#[derive(Debug)]
pub struct Input {
    pub numbers: Vec<usize>,
    pub layouts: Vec<Layout>,
}

pub fn parse_input<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Input> {
    fn numbers<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<usize>> {
        separated_list1(char(','), parse_integral_nonnegative())
    }

    fn layout<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Layout> {
        map(
            separated_list1(
                newline,
                preceded(
                    many0(char(' ')),
                    separated_list1(many1(char(' ')), parse_integral_nonnegative()),
                ),
            ),
            |lines| {
                let mut ret = HashMap::new();
                for (y, line) in lines.iter().enumerate() {
                    for (x, n) in line.iter().enumerate() {
                        ret.insert(*n, (x, y));
                    }
                }
                ret
            },
        )
    }

    all_consuming(map(
        separated_pair(
            newline_terminated(numbers()),
            newline,
            separated_list1(newline, newline_terminated(layout())),
        ),
        |(numbers, layouts)| Input { numbers, layouts },
    ))
}

/// A `Board` is a layout together with its state; i.e., the number of
/// spaces marked in each column and row.
pub struct Board {
    pub layout: Layout,
    pub columns: Vec<usize>,
    pub rows: Vec<usize>,
}

impl Board {
    pub fn new(layout: Layout) -> Self {
        let cs = layout.values().map(|(x, _)| x).max().unwrap() + 1;
        let rs = layout.values().map(|(_, y)| y).max().unwrap() + 1;
        Board {
            layout,
            columns: vec![0; cs],
            rows: vec![0; rs],
        }
    }

    /// The dimension (width/length) of the board.
    pub fn dim(&self) -> usize {
        self.rows.len()
    }

    /// If the board has a given number, mark it and determine whether
    /// this board has won.
    ///
    /// We assume that a given number will never be marked more than
    /// once.
    pub fn mark_and_check(&mut self, n: usize) -> bool {
        match self.layout.get(&n) {
            None => false,
            Some((i, j)) => {
                self.columns[*i] += 1;
                self.rows[*j] += 1;
                self.columns[*i] == self.dim() || self.rows[*j] == self.dim()
            }
        }
    }
}

/// An active `Game` contains a sequence of numbers to be called and a
/// sequence of boards.
pub struct Game {
    pub numbers: Vec<usize>,
    pub boards: Vec<Board>,
}

impl Game {
    pub fn from_input(input: Input) -> Game {
        Game {
            numbers: input.numbers,
            boards: input.layouts.into_iter().map(Board::new).collect_vec(),
        }
    }
}
