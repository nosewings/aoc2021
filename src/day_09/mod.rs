use itertools::Itertools;
use ndarray::Array2;
use nom::character::complete::{newline, satisfy};
use nom::character::is_digit;
use nom::combinator::{all_consuming, map_res};
use nom::multi::{many1, separated_list1};
use nom::IResult;

use crate::{newline_terminated, Shape2};

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

pub fn adjacent_indices(
    (w, h): (usize, usize),
    (i, j): (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    let l = i.checked_sub(1).map(|k| (k, j));
    let r = i.checked_add(1).filter(|&k| k < w).map(|k| (k, j));
    let u = j.checked_sub(1).map(|k| (i, k));
    let d = j.checked_add(1).filter(|&k| k < h).map(|k| (i, k));
    [l, r, u, d].into_iter().flatten()
}

#[allow(clippy::needless_lifetimes)]
pub fn lows<'a>(input: &'a Array2<u32>) -> impl Iterator<Item = (usize, usize)> + 'a {
    input
        .indexed_iter()
        .filter(|(ix, &h)| {
            adjacent_indices(input.shape2(), *ix).all(|ix| input.get(ix).map_or(true, |&x| h < x))
        })
        .map(|p| p.0)
}
