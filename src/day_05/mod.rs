use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

use crate::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32,
}

#[derive(PartialEq, Eq)]
pub enum VentsClass {
    Horizontal,
    Vertical,
    Diagonal,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Vents {
    pub start: Coordinate,
    pub end: Coordinate,
}

impl Vents {
    pub fn class(&self) -> VentsClass {
        if self.start.x == self.end.x {
            VentsClass::Vertical
        } else if self.start.y == self.end.y {
            VentsClass::Horizontal
        } else {
            VentsClass::Diagonal
        }
    }

    pub fn covered(&self) -> impl Iterator<Item = Coordinate> {
        let ret: Box<dyn Iterator<Item = Coordinate>> = match self.class() {
            VentsClass::Vertical => Box::new(
                std::iter::repeat(self.start.x)
                    .zip(riar(self.start.y..=self.end.y))
                    .map(|(x, y)| Coordinate { x, y }),
            ),
            VentsClass::Horizontal => Box::new(
                riar(self.start.x..=self.end.x)
                    .zip(std::iter::repeat(self.start.y))
                    .map(|(x, y)| Coordinate { x, y }),
            ),
            VentsClass::Diagonal => Box::new(
                riar(self.start.x..=self.end.x)
                    .zip(riar(self.start.y..=self.end.y))
                    .map(|(x, y)| Coordinate { x, y }),
            ),
        };
        ret
    }
}

pub fn parse_vents<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Vents>> {
    fn coordinate<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Coordinate> {
        map(
            separated_pair(parse_u32(), char(','), parse_u32()),
            |(x, y)| Coordinate { x, y },
        )
    }

    fn line<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vents> {
        map(
            separated_pair(coordinate(), tag(" -> "), coordinate()),
            |(start, end)| Vents { start, end },
        )
    }

    all_consuming(newline_terminated(separated_list1(char('\n'), line())))
}
