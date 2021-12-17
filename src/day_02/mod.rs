use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

use crate::*;

pub enum Direction {
    Forward,
    Down,
    Up,
}
use Direction::*;

pub struct Command {
    pub direction: Direction,
    pub magnitude: i32,
}

pub fn parse_command<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Command>> {
    fn direction<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Direction> {
        alt((
            map(tag("forward"), |_| Forward),
            map(tag("down"), |_| Down),
            map(tag("up"), |_| Up),
        ))
    }

    fn command<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Command> {
        map(
            separated_pair(direction(), char(' '), parse_i32_nn()),
            |(direction, magnitude)| Command {
                direction,
                magnitude,
            },
        )
    }

    all_consuming(newline_terminated(separated_list1(char('\n'), command())))
}
