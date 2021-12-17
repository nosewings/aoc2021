use std::iter::FromIterator;
use std::mem::size_of;

use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{all_consuming, map_res};
use nom::multi::{many1, separated_list1};
use nom::IResult;

use crate::newline_terminated;

pub const LINE_LEN_BOUND: usize = 8 * size_of::<u32>();

#[derive(Clone)]
pub struct Input {
    pub line_length: usize,
    pub lines: Vec<u32>,
}

pub fn parse_input<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Input> {
    pub fn binary<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, (u32, usize)> {
        map_res(many1(alt((char('0'), char('1')))), |v| {
            u32::from_str_radix(&String::from_iter(v.iter()), 2).map(|n| (n, v.len()))
        })
    }
    all_consuming(newline_terminated(nom::combinator::map(
        separated_list1(char('\n'), binary()),
        |v| {
            let (ls, ns): (Vec<_>, Vec<_>) = v.iter().cloned().unzip();
            assert!(ns.iter().all(|n| *n == ns[0]));
            Input {
                line_length: ns[0],
                lines: ls,
            }
        },
    )))
}
