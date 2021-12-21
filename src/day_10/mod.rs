use nom::character::complete::{anychar, newline};
use nom::combinator::{all_consuming, map_opt};
use nom::multi::{many1, separated_list1};
use nom::IResult;

use crate::newline_terminated;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum BracketType {
    Paren,
    Bracket,
    Brace,
    Angle,
}
use BracketType::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Character {
    Open(BracketType),
    Close(BracketType),
}
use Character::*;

impl Character {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '(' => Some(Open(Paren)),
            ')' => Some(Close(Paren)),
            '[' => Some(Open(Bracket)),
            ']' => Some(Close(Bracket)),
            '{' => Some(Open(Brace)),
            '}' => Some(Close(Brace)),
            '<' => Some(Open(Angle)),
            '>' => Some(Close(Angle)),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Open(Paren) => '(',
            Close(Paren) => ')',
            Open(Bracket) => '[',
            Close(Bracket) => ']',
            Open(Brace) => '{',
            Close(Brace) => '}',
            Open(Angle) => '<',
            Close(Angle) => '>',
        }
    }
}

pub fn parse_input<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Vec<Character>>> {
    fn character<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Character> {
        map_opt(anychar, Character::from_char)
    }

    all_consuming(newline_terminated(separated_list1(
        newline,
        many1(character()),
    )))
}

pub enum Outcome {
    Corrupted(BracketType),
    Incomplete(Vec<BracketType>),
}
use Outcome::*;

pub fn scan(line: &[Character]) -> Outcome {
    let mut stack = Vec::new();
    for character in line.iter().copied() {
        match (stack.last(), character) {
            (_, Open(c)) => {
                stack.push(c);
            }
            (Some(&c1), Close(c2)) if c1 == c2 => {
                stack.pop();
            }
            (_, Close(c)) => {
                return Corrupted(c);
            }
        }
    }
    Incomplete(stack)
}
