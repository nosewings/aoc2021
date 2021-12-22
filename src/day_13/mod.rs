use std::collections::HashSet;

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, newline};
use nom::combinator::{all_consuming, map, value};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

use crate::{newline_terminated, parse_integral_nonnegative};

#[derive(Clone, Copy)]
enum Axis {
    X,
    Y,
}
use Axis::*;

pub enum Fold {
    Horizontal(u32),
    Vertical(u32),
}
use Fold::*;

impl Fold {
    pub fn execute(&self, dots: &mut HashSet<(u32, u32)>) {
        dots.drain_filter(|(x, y)| match self {
            Horizontal(n) => y > n,
            Vertical(n) => x > n,
        })
        .map(|(x, y)| match self {
            Horizontal(n) => (x, 2 * n - y),
            Vertical(n) => (2 * n - x, y),
        })
        .collect_vec()
        .into_iter()
        .for_each(|p| {
            dots.insert(p);
        });
    }
}

pub struct Input {
    pub dots: HashSet<(u32, u32)>,
    pub folds: Vec<Fold>,
}

pub fn parse_input<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Input> {
    fn dot<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, (u32, u32)> {
        separated_pair(
            parse_integral_nonnegative(),
            char(','),
            parse_integral_nonnegative(),
        )
    }

    fn axis<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Axis> {
        alt((value(X, char('x')), value(Y, char('y'))))
    }

    fn fold<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Fold> {
        preceded(
            tag("fold along "),
            map(
                separated_pair(axis(), char('='), parse_integral_nonnegative()),
                |(axis, n)| match axis {
                    X => Vertical(n),
                    Y => Horizontal(n),
                },
            ),
        )
    }

    map(
        all_consuming(separated_pair(
            newline_terminated(separated_list1(newline, dot())),
            newline,
            newline_terminated(separated_list1(newline, fold())),
        )),
        |(dots, folds): (Vec<(u32, u32)>, Vec<Fold>)| Input {
            dots: HashSet::from_iter(dots.into_iter()),
            folds,
        },
    )
}
