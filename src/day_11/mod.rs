use itertools::Itertools;
use ndarray::Array2;
use nom::character::complete::{anychar, newline};
use nom::combinator::{all_consuming, map_res};
use nom::multi::{many1, separated_list1};
use nom::IResult;

use crate::{newline_terminated, Array2Ext, FoldMap};

pub fn parse_input<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Array2<u32>> {
    fn digit<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, u32> {
        map_res(anychar, |c| str::parse(&format!("{}", c)))
    }

    all_consuming(newline_terminated(map_res(
        separated_list1(newline, many1(digit())),
        Array2::from_rows,
    )))
}

pub fn check_for_flash(grid: &mut Array2<u32>, ix: (usize, usize)) -> u32 {
    if grid[ix] <= 9 {
        return 0;
    }
    grid[ix] = 0;
    1 + grid.neighbor_indices(ix).into_iter().fold_map(|ix| {
        if grid[ix] == 0 {
            0
        } else {
            grid[ix] += 1;
            check_for_flash(grid, ix)
        }
    })
}

pub fn step(grid: &mut Array2<u32>) -> u32 {
    grid.map_inplace(|x| *x += 1);
    grid.indexed_iter()
        .map(|p| p.0)
        .collect_vec()
        .into_iter()
        .fold_map(|ix| check_for_flash(grid, ix))
}
