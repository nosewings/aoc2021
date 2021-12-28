use combine::error::UnexpectedParse;
use combine::parser::char::{hex_digit, newline};
use combine::parser::function::env_parser;
use combine::parser::range::take;
use combine::{
    any, count_min_max, eof, many, one_of, parser, unexpected_any, value, Parser, StdParseResult,
    Stream,
};
use itertools::Itertools;
use ArbOp::*;
use BinOp::*;
use Binary::*;
use Payload::*;

use crate::{FoldMap, FoldMapOption};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Binary {
    Zero,
    One,
}

impl Binary {
    fn try_to_u8(bits: &[Binary]) -> Option<u8> {
        if bits.len() >= u8::BITS as usize {
            return None;
        }
        Some(
            bits.iter()
                .copied()
                .enumerate()
                .fold_map(|(i, b)| {
                    let j = bits.len() - i - 1;
                    frunk::semigroup::Any((u8::from(b) << j) & (1 << j))
                })
                .0,
        )
    }

    fn try_to_u64(bits: &[Binary]) -> Option<u64> {
        if bits.len() >= u64::BITS as usize {
            return None;
        }
        Some(
            bits.iter()
                .copied()
                .enumerate()
                .fold_map(|(i, b)| {
                    let j = bits.len() - i - 1;
                    frunk::semigroup::Any((u64::from(b) << j) & (1 << j))
                })
                .0,
        )
    }

    fn try_to_usize(bits: &[Binary]) -> Option<usize> {
        if bits.len() >= usize::BITS as usize {
            return None;
        }
        Some(
            bits.iter()
                .copied()
                .enumerate()
                .fold_map(|(i, b)| {
                    let j = bits.len() - i - 1;
                    frunk::semigroup::Any((usize::from(b) << j) & (1 << j))
                })
                .0,
        )
    }
}

impl From<Binary> for u8 {
    fn from(b: Binary) -> Self {
        match b {
            Zero => 0,
            One => 1,
        }
    }
}

impl From<Binary> for u64 {
    fn from(b: Binary) -> Self {
        match b {
            Zero => 0,
            One => 1,
        }
    }
}

impl From<Binary> for usize {
    fn from(b: Binary) -> Self {
        match b {
            Zero => 0,
            One => 1,
        }
    }
}

impl TryFrom<u8> for Binary {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Zero),
            1 => Ok(One),
            _ => Err(()),
        }
    }
}

fn unhex<Input>() -> impl Parser<Input, Output = [Binary; 4]>
where
    Input: Stream<Token = char>,
{
    hex_digit().map(|c| {
        // This can't fail: the input character has already passed
        // through the hex_digit parser.
        let n = u8::from_str_radix(&c.to_string(), 16).unwrap_or_else(|_| unreachable!());
        let mut ret = [Zero; 4];
        for (i, b) in ret.iter_mut().enumerate() {
            // This can't fail: AND-ing with 1 will always produce
            // either a 0 or a 1.
            *b = Binary::try_from((n >> (3 - i)) & 1).unwrap_or_else(|_| unreachable!());
        }
        ret
    })
}

#[derive(Clone, Debug)]
pub struct Packet {
    pub version: u8,
    pub payload: Payload,
}

impl Packet {
    pub fn subpackets(&self) -> impl Iterator<Item = &Self> {
        std::iter::once(self).chain(self.payload.packets())
    }

    pub fn eval(&self) -> u64 {
        self.payload.eval()
    }
}

#[derive(Clone, Debug)]
pub enum Payload {
    Literal(u64),
    Arb(ArbOp, Vec<Packet>),
    Bin(BinOp, Box<Packet>, Box<Packet>),
}

impl Payload {
    pub fn packets(&self) -> impl Iterator<Item = &Packet> {
        let ret: Box<dyn Iterator<Item = _>> = match self {
            Literal(_) => Box::new(std::iter::empty()),
            Arb(_, args) => Box::new(args.iter().flat_map(Packet::subpackets)),
            Bin(_, arg1, arg2) => Box::new(arg1.subpackets().chain(arg2.subpackets())),
        };
        ret
    }

    pub fn eval(&self) -> u64 {
        match self {
            Literal(n) => *n,
            Arb(op, args) => op.eval(args.iter().map(Packet::eval)),
            Bin(op, arg1, arg2) => op.eval(arg1.eval(), arg2.eval()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ArbOp {
    Sum,
    Prod,
    Min,
    Max,
}

impl ArbOp {
    pub fn eval<I>(&self, args: I) -> u64
    where
        I: Iterator<Item = u64>,
    {
        match self {
            Sum => args.fold_map(|x| x),
            Prod => args.fold_map(frunk::semigroup::Product).0,
            Min => args.fold_map_option(frunk::semigroup::Min).unwrap().0,
            Max => args.fold_map_option(frunk::semigroup::Max).unwrap().0,
        }
    }
}

impl TryFrom<u8> for ArbOp {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Sum),
            1 => Ok(Prod),
            2 => Ok(Min),
            3 => Ok(Max),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BinOp {
    Gt,
    Lt,
    Eq,
}

impl BinOp {
    pub fn eval(&self, arg1: u64, arg2: u64) -> u64 {
        match self {
            Gt => (arg1 > arg2) as u64,
            Lt => (arg1 < arg2) as u64,
            Eq => (arg1 == arg2) as u64,
        }
    }
}

impl TryFrom<u8> for BinOp {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            5 => Ok(Gt),
            6 => Ok(Lt),
            7 => Ok(Eq),
            _ => Err(()),
        }
    }
}

fn literal<'a>(id: u8) -> impl Parser<&'a [Binary], Output = Payload> {
    if id != 4 {
        return unexpected_any("bad opcode for literal").left();
    }
    fn go<'a>(
        mut env: Vec<Binary>,
        input: &mut &'a [Binary],
    ) -> StdParseResult<Vec<Binary>, &'a [Binary]> {
        let (bs, c) = take(5).parse_stream(input).into_result()?;
        env.extend(bs[1..].iter());
        if bs[0] == Zero {
            Ok((env, c))
        } else {
            c.combine(|_| go(env, input))
        }
    }

    let buf = Vec::new();
    env_parser(buf, go)
        .flat_map(|buf| {
            Binary::try_to_u64(&buf)
                .map(Literal)
                .ok_or(UnexpectedParse::Unexpected)
        })
        .right()
}

fn binary_to_usize<'a>(n: usize) -> impl Parser<&'a [Binary], Output = usize> {
    take(n).then(|size: &'a [Binary]| match Binary::try_to_usize(size) {
        Some(size) => value(size).left(),
        None => unexpected_any("small word size").right(),
    })
}

fn arbop<'a>(id: u8) -> impl Parser<&'a [Binary], Output = Payload> {
    let op = match ArbOp::try_from(id) {
        Ok(op) => op,
        Err(_) => return unexpected_any("bad opcode for arbop").left(),
    };
    any()
        .then(move |mode| match mode {
            Zero => binary_to_usize(15)
                .then(move |size| {
                    take(size)
                        .flat_map(|bs| many(packet()).and(eof()).parse(bs).map(|p| p.0 .0))
                        .map(move |bs| Arb(op, bs))
                })
                .left(),
            One => binary_to_usize(11)
                .then(move |size| count_min_max(size, size, packet()).map(move |bs| Arb(op, bs)))
                .right(),
        })
        .right()
}

fn binop<'a>(id: u8) -> impl Parser<&'a [Binary], Output = Payload> {
    let op = match BinOp::try_from(id) {
        Ok(op) => op,
        Err(_) => return unexpected_any("bad opcode for binop").left(),
    };
    any()
        .then(move |mode| match mode {
            Zero => binary_to_usize(15)
                .then(move |size| {
                    take(size).flat_map(move |bs| {
                        packet()
                            .and(packet())
                            .and(eof())
                            .parse(bs)
                            .map(move |(((p1, p2), _), _)| Bin(op, Box::new(p1), Box::new(p2)))
                    })
                })
                .left(),
            One => binary_to_usize(11)
                .then(move |size| match size {
                    2 => packet()
                        .and(packet())
                        .map(move |(p1, p2)| Bin(op, Box::new(p1), Box::new(p2)))
                        .left(),
                    _ => unexpected_any("wrong number of arguments for binary operator").right(),
                })
                .right(),
        })
        .right()
}

fn packet_<'a>() -> impl Parser<&'a [Binary], Output = Packet> {
    take(3)
        .and(take(3))
        .then(|(version, id): (&'a [Binary], &'a [Binary])| {
            // These can't fail: both arguments are guaranteed to have
            // length 3 <= 8.
            let version = Binary::try_to_u8(version).unwrap_or_else(|| unreachable!());
            let id = Binary::try_to_u8(id).unwrap_or_else(|| unreachable!());
            literal(id)
                .or(arbop(id))
                .or(binop(id))
                .map(move |payload| Packet { version, payload })
        })
}

parser! {
    fn packet['a]()(&'a [Binary]) -> Packet where [] {
        packet_()
    }
}

fn top_level_packet<'a>() -> impl Parser<&'a [Binary], Output = Packet> {
    packet()
        .and(count_min_max::<Vec<_>, _, _>(0, 7, one_of([Zero])))
        .and(eof())
        .map(|p| p.0 .0)
}

fn packet_from_chars<Input>() -> impl Parser<Input, Output = Packet>
where
    Input: Stream<Token = char, Error = UnexpectedParse>,
    Input::Position: Default,
{
    many::<Vec<_>, _, _>(unhex())
        .and(newline())
        .and(eof())
        .map(|p| p.0 .0.concat())
        .flat_map(|bs| top_level_packet().parse(&bs).map(|p| p.0))
}

#[allow(clippy::let_and_return)]
pub fn parse_packet() -> impl Fn(&str) -> Result<(String, Packet), UnexpectedParse> {
    |s| {
        let chrs = s.chars().collect_vec();
        let x = packet_from_chars()
            .parse(&chrs[..])
            .map(|(p, s)| (s.iter().collect(), p));
        x
    }
}
