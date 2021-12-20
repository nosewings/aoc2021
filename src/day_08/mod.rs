use std::convert::TryFrom;
use std::iter::FromIterator;

use enumset::{EnumSet, EnumSetType};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{all_consuming, map, map_res, value};
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use nom::IResult;

use crate::*;

#[derive(Debug, EnumSetType)]
#[repr(usize)]
pub enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}
use Segment::*;

pub type Patterns = [EnumSet<Segment>; 10];
pub type Outputs = [EnumSet<Segment>; 4];

pub struct Displays {
    pub patterns: Patterns,
    pub outputs: Outputs,
}

pub fn parse_input<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Displays>> {
    fn segment<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Segment> {
        alt((
            value(A, char('a')),
            value(B, char('b')),
            value(C, char('c')),
            value(D, char('d')),
            value(E, char('e')),
            value(F, char('f')),
            value(G, char('g')),
        ))
    }

    fn digits<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<EnumSet<Segment>>> {
        separated_list1(char(' '), map(many1(segment()), EnumSet::from_iter))
    }

    fn displays<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Displays> {
        map(
            separated_pair(
                map_res(digits(), Patterns::try_from),
                tag(" | "),
                map_res(digits(), Outputs::try_from),
            ),
            |(patterns, outputs)| Displays { patterns, outputs },
        )
    }

    all_consuming(newline_terminated(separated_list1(char('\n'), displays())))
}
