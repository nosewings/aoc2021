use nom::character::complete::char;
use nom::combinator::all_consuming;
use nom::multi::separated_list1;
use nom::IResult;

use crate::*;

pub fn parse_input<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<u32>> {
    all_consuming(newline_terminated(separated_list1(
        char(','),
        parse_integral_nonnegative(),
    )))
}
