use std::collections::HashMap;

use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline, satisfy};
use nom::combinator::{all_consuming, map};
use nom::multi::separated_list1;
use nom::sequence::{pair, separated_pair};
use nom::{AsChar, IResult, InputIter};

use crate::newline_terminated;

pub struct Input {
    pub template: Vec<char>,
    pub rules: HashMap<(char, char), char>,
}

pub fn parse_input<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Input> {
    fn template<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<char>> {
        map(alpha1, |s: &'a str| s.iter_elements().collect_vec())
    }

    fn rules<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, HashMap<(char, char), char>> {
        map(
            separated_list1(
                newline,
                separated_pair(
                    pair(satisfy(|c| c.is_alpha()), satisfy(|c| c.is_alpha())),
                    tag(" -> "),
                    satisfy(|c| c.is_alpha()),
                ),
            ),
            |lines: Vec<((char, char), char)>| HashMap::from_iter(lines.into_iter()),
        )
    }

    map(
        all_consuming(separated_pair(
            newline_terminated(template()),
            newline,
            newline_terminated(rules()),
        )),
        |(template, rules)| Input { template, rules },
    )
}
