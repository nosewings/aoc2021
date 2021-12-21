use itertools::Itertools;
use ndarray::Array2;
use nom::character::complete::{newline, satisfy};
use nom::character::is_digit;
use nom::combinator::{all_consuming, map_res};
use nom::multi::{many1, separated_list1};
use nom::IResult;

use crate::{newline_terminated, Array2Ext};

pub fn parse_input<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Array2<u32>> {
    fn digit<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, u32> {
        map_res(satisfy(|c| is_digit(c as u8)), |c| {
            str::parse(&format!("{}", c))
        })
    }

    all_consuming(newline_terminated(map_res(
        separated_list1(newline, many1(digit())),
        |lines| {
            let shape = (lines.len(), lines[0].len());
            let lines = lines.into_iter().flatten().collect_vec();
            Array2::from_shape_vec(shape, lines)
        },
    )))
}

pub fn lows(input: &Array2<u32>) -> impl Iterator<Item = (usize, usize)> + '_ {
    input
        .indexed_iter()
        .filter(|(ix, &h)| {
            input
                .neighbor_indices(*ix)
                .into_iter()
                .all(|ix| input.get(ix).map_or(true, |&x| h < x))
        })
        .map(|p| p.0)
}
