use combine::error::StringStreamError;
use combine::Parser;
use itertools::Itertools;

use self::cuboid::Cuboid;
use crate::day_22::vector::Vector;

pub mod cuboid;
pub mod vector;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Step {
    pub region: Cuboid,
    pub value: bool,
}

impl Step {
    pub fn volume(&self) -> i64 {
        (if self.value { 1 } else { -1 }) * self.region.volume()
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Input {
    pub steps: Vec<Step>,
}

impl Input {
    pub fn solve(self) -> i64 {
        let mut decomposed_steps = Vec::<Step>::new();
        for s in self.steps {
            decomposed_steps.extend(
                decomposed_steps
                    .iter()
                    .flat_map(|d| {
                        let region = s.region.intersect(d.region)?;
                        let value = !d.value;
                        Some(Step { region, value })
                    })
                    .collect_vec(),
            );
            if s.value {
                decomposed_steps.push(s);
            }
        }
        decomposed_steps.iter().map(Step::volume).sum()
    }
}

pub fn parse_input<'a>() -> impl Parser<&'a str, Output = Input> {
    use combine::parser::char::*;
    use combine::*;

    use crate::combine_parse_integral;

    fn bool<'a>() -> impl Parser<&'a str, Output = bool> {
        choice!(
            attempt(string("on")).with(value(true)),
            string("off").with(value(false))
        )
    }

    fn range<'a>(label: char) -> impl Parser<&'a str, Output = (i64, i64)> {
        char(label)
            .skip(char('='))
            .with(combine_parse_integral())
            .skip(string(".."))
            .and(combine_parse_integral())
    }

    fn step<'a>() -> impl Parser<&'a str, Output = Step> {
        bool()
            .skip(char(' '))
            .and(range('x'))
            .skip(char(','))
            .and(range('y'))
            .skip(char(','))
            .and(range('z'))
            .flat_map(|(((value, (x_lo, x_hi)), (y_lo, y_hi)), (z_lo, z_hi))| {
                let lo = Vector::from([x_lo, y_lo, z_lo]);
                let hi = Vector::from([x_hi, y_hi, z_hi]).add_scalar(1);
                let region = Cuboid::new(lo, hi).ok_or(StringStreamError::UnexpectedParse)?;
                Ok(Step { region, value })
            })
    }

    fn steps<'a>() -> impl Parser<&'a str, Output = Vec<Step>> {
        sep_end_by(step(), newline())
    }

    steps().skip(eof()).map(|steps| Input { steps })
}
