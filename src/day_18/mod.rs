mod tree;
use std::ops::Add;

use combine::error::StringStreamError;
use combine::Parser;

use self::tree::*;
use self::Node::*;

#[derive(Clone, Debug)]
pub struct Number(Tree);

impl Number {
    pub fn repr(&self) -> String {
        fn go(tree: &Tree) -> String {
            match tree {
                box Leaf(n) => n.to_string(),
                box Branch(l, r) => format!("[{},{}]", go(l), go(r)),
            }
        }
        go(&self.0)
    }

    // I decided I was bored of nice composable functional stuff. :D
    fn reduce(self) -> Self {
        let mut zipper = Zipper::from(self.0);
        // Outer loop: repeat explode/split steps until we reach a
        // fixed point.
        'outer: loop {
            // Explode loop.
            loop {
                if let box Branch(box Leaf(l), box Leaf(r)) = zipper.focus() {
                    if zipper.depth() >= 4 {
                        let l = *l;
                        let r = *r;
                        if let Some(n) = zipper.left_leaf_mut() {
                            *n += l;
                        }
                        if let Some(n) = zipper.right_leaf_mut() {
                            *n += r;
                        }
                        *zipper.focus_mut() = box Leaf(0);
                    }
                }
                zipper.focus_next_depth_first();
                if zipper.at_top() {
                    break;
                }
            }
            // Split loop.
            zipper.focus_top();
            while let Some(n) = zipper.focus_next_leaf_depth_first() {
                if *n >= 10 {
                    let l = *n / 2;
                    let r = *n - l;
                    *zipper.focus_mut() = box Branch(box Leaf(l), box Leaf(r));
                    // We have to start looking for explodes again.
                    continue 'outer;
                }
            }
            break;
        }
        Self(Tree::from(zipper))
    }

    pub fn magnitude(&self) -> u32 {
        fn go(tree: &Tree) -> u32 {
            match tree {
                box Leaf(n) => *n,
                box Branch(l, r) => 3 * go(l) + 2 * go(r),
            }
        }
        go(&self.0)
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Number) -> Self {
        Number(box Branch(self.0, rhs.0)).reduce()
    }
}

fn parse_tree_<'a>() -> impl Parser<&'a str, Output = Tree> {
    use combine::between;
    use combine::parser::char::char;

    crate::combine_parse_integral_nonnegative()
        .map(|n| box Leaf(n))
        .or(between(
            char('['),
            char(']'),
            parse_tree()
                .skip(char(','))
                .and(parse_tree())
                .map(|(l, r)| box Branch(l, r)),
        ))
}

combine::parser! {
    fn parse_tree['a]()(&'a str) -> Tree where [] {
        parse_tree_()
    }
}

fn parse_number<'a>() -> impl Parser<&'a str, Output = Number> {
    parse_tree().map(Number)
}

fn parse_numbers<'a>() -> impl Parser<&'a str, Output = Vec<Number>> {
    use combine::parser::char::newline;
    use combine::{eof, sep_end_by1};
    sep_end_by1(parse_number(), newline()).skip(eof())
}

#[allow(clippy::let_and_return)]
pub fn parse_input() -> impl Fn(&str) -> Result<(String, Vec<Number>), StringStreamError> {
    |s| {
        let chrs = s.chars().collect::<String>();
        let x = parse_numbers()
            .parse(&chrs[..])
            .map(|(t, s)| (s.to_string(), t));
        x
    }
}
