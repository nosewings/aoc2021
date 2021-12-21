#![feature(step_trait)]

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;

use std::io::Read;
use std::iter::Step;
use std::ops::RangeInclusive;
use std::str::FromStr;

use frunk::monoid::Monoid;
use ndarray::Array2;
use nom::character::complete::{digit1, newline};
use nom::combinator::{all_consuming, map_res};
use nom::sequence::terminated;
use nom::IResult;

pub fn read_input(n: u32) -> String {
    let args = std::env::args().collect::<Vec<_>>();
    let file_name = match args[..] {
        [_, ref input_file] => input_file.clone(),
        [_] => format!("./inputs/{:>02}.txt", n),
        _ => panic!("invalid arguments"),
    };

    if file_name == "-" {
        let mut ret = String::new();
        std::io::stdin()
            .read_to_string(&mut ret)
            .expect("error while reading input");
        ret
    } else {
        std::fs::read_to_string(file_name).expect("error while reading input")
    }
}

#[macro_export]
macro_rules! make_main {
    ($day:literal, $parse:ident, $run:ident) => {
        fn main() {
            use aoc2021::read_input;
            let s = read_input($day);
            let (_, v) = $parse()(&s).expect("error while parsing input");
            println!("{}", $run(v));
        }
    };
}

#[macro_export]
macro_rules! make_test {
    ($day: literal, $part:literal, $parse:ident, $run:ident, $expected:literal) => {
        #[cfg(test)]
        mod test {
            use aoc2021::read_input;
            use paste::paste;

            use super::{$parse, $run};
            paste! {
                #[test]
                fn [<test_ $day _ $part>]() {
                    let s = read_input($day);
                    let (_, p) = $parse()(&s).expect("error while parsing input");
                    let v = $run(p);
                    assert_eq!(v, $expected);
                }
            }
        }
    };
}

/// **R**ange **i**nclusive **a**uto **r**eversed.
///
/// This function assumes that the index type has concordant `Ord` and
/// `Step` implementations.
pub fn riar<'a, Idx>(range: RangeInclusive<Idx>) -> impl Iterator<Item = Idx> + 'a
where
    Idx: Ord + Step + 'a,
{
    let (start, end) = range.into_inner();
    let ret: Box<dyn Iterator<Item = Idx> + 'a> = if start <= end {
        Box::new(start..=end)
    } else {
        Box::new((end..=start).rev())
    };
    ret
}

/// A trait for monkey-patching Haskell's `foldMap` onto Rust's
/// iterators.
pub trait FoldMap {
    /// The item type for this type.
    type Item;

    /// Map the items to a `Monoid`, and them combine the results
    /// monoidally.
    fn fold_map<M, F>(self, f: F) -> M
    where
        M: Monoid,
        F: FnMut(Self::Item) -> M;
}

impl<I> FoldMap for I
where
    I: Iterator,
{
    type Item = <Self as Iterator>::Item;

    fn fold_map<M, F>(self, f: F) -> M
    where
        M: Monoid,
        F: FnMut(Self::Item) -> M,
    {
        self.map(f).fold(M::empty(), |x, y| x.combine(&y))
    }
}

pub trait Array2Ext {
    fn shape2(&self) -> (usize, usize);
    fn cardinal_neighbor_indices(&self, ix: (usize, usize)) -> Vec<(usize, usize)>;
    fn neighbor_indices(&self, ix: (usize, usize)) -> Vec<(usize, usize)>;
}

impl<T> Array2Ext for Array2<T> {
    fn shape2(&self) -> (usize, usize) {
        type Sh = [usize; 2];
        let [w, h] = Sh::try_from(self.shape()).unwrap();
        (w, h)
    }

    fn cardinal_neighbor_indices(&self, (i, j): (usize, usize)) -> Vec<(usize, usize)> {
        let (w, h) = self.shape2();
        let l = i.checked_sub(1);
        let r = i.checked_add(1).filter(|&k| k < w);
        let u = j.checked_sub(1);
        let d = j.checked_add(1).filter(|&k| k < h);
        let mut ret = Vec::new();
        if let Some(k) = r {
            ret.push((k, j))
        }
        if let Some(k) = l {
            ret.push((k, j))
        }
        if let Some(k) = u {
            ret.push((i, k))
        }
        if let Some(k) = d {
            ret.push((i, k))
        }
        ret
    }

    fn neighbor_indices(&self, (i, j): (usize, usize)) -> Vec<(usize, usize)> {
        let (w, h) = self.shape2();
        let l = i.checked_sub(1);
        let r = i.checked_add(1).filter(|&k| k < w);
        let u = j.checked_sub(1);
        let d = j.checked_add(1).filter(|&k| k < h);
        let mut ret = Vec::new();
        if let Some(k) = r {
            ret.push((k, j))
        }
        if let Some(k) = l {
            ret.push((k, j))
        }
        if let Some(k) = u {
            ret.push((i, k))
        }
        if let Some(k) = d {
            ret.push((i, k))
        }
        if let Some(ix) = r.zip(u) {
            ret.push(ix)
        }
        if let Some(ix) = r.zip(d) {
            ret.push(ix)
        }
        if let Some(ix) = l.zip(u) {
            ret.push(ix)
        }
        if let Some(ix) = l.zip(d) {
            ret.push(ix)
        }
        ret
    }
}

pub fn parse_integral_nonnegative<'a, T>() -> impl FnMut(&'a str) -> IResult<&'a str, T>
where
    T: FromStr,
{
    map_res(digit1, str::parse)
}

pub fn newline_terminated<'a, A, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, A>
where
    F: FnMut(&'a str) -> IResult<&'a str, A>,
{
    terminated(f, newline)
}
