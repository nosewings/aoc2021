use std::collections::HashMap;

use nom::character::complete::{alpha1, char, newline};
use nom::combinator::{all_consuming, map, map_opt};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{IResult, InputIter};

use crate::newline_terminated;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Size {
    Big,
    Small,
}
use Size::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Cave<'a> {
    name: &'a str,
    size: Size,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Edge<'a> {
    start: Cave<'a>,
    end: Cave<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub size: Size,
    pub edges: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Graph {
    pub start: usize,
    pub end: usize,
    pub nodes: Vec<Node>,
}

pub fn parse_input<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Graph> {
    fn cave<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Cave> {
        map_opt(alpha1, |name: &'a str| {
            let size = if name.iter_elements().all(char::is_uppercase) {
                Some(Big)
            } else if name.iter_elements().all(char::is_lowercase) {
                Some(Small)
            } else {
                None
            }?;
            Some(Cave { name, size })
        })
    }

    fn edge<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Edge> {
        map(separated_pair(cave(), char('-'), cave()), |(start, end)| {
            Edge { start, end }
        })
    }

    map_opt(
        all_consuming(newline_terminated(separated_list1(newline, edge()))),
        |edges| {
            let mut mapping = HashMap::new();
            let mut nodes = Vec::new();

            fn get_index<'a>(
                mapping: &mut HashMap<&'a str, usize>,
                nodes: &mut Vec<Node>,
                key: Cave<'a>,
            ) -> usize {
                *mapping.entry(key.name).or_insert_with(|| {
                    let i = nodes.len();
                    nodes.push(Node {
                        size: key.size,
                        edges: Vec::new(),
                    });
                    i
                })
            }

            for edge in edges {
                let i = get_index(&mut mapping, &mut nodes, edge.start);
                let j = get_index(&mut mapping, &mut nodes, edge.end);
                nodes[i].edges.push(j);
                nodes[j].edges.push(i);
            }

            Some(Graph {
                start: *mapping.get("start")?,
                end: *mapping.get("end")?,
                nodes,
            })
        },
    )
}
