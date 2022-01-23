use combine::Parser;

pub struct Input {
    pub positions: [u32; 2],
}

pub fn parse_input<'a>() -> impl Parser<&'a str, Output = Input> {
    use combine::parser::char::*;
    use combine::*;

    use crate::combine_parse_integral_nonnegative;

    string("Player 1 starting position: ")
        .with(combine_parse_integral_nonnegative())
        .skip(newline())
        .skip(string("Player 2 starting position: "))
        .and(combine_parse_integral_nonnegative())
        .skip(newline())
        .skip(eof())
        .map(|(p1, p2)| Input {
            positions: [p1, p2],
        })
}

pub struct Player {
    pub position: u32,
    pub score: u32,
}

impl Player {
    pub fn advance(&mut self, n: u32) -> bool {
        self.position = (((self.position - 1) + n) % 10) + 1;
        self.score += self.position;
        self.score >= 1000
    }
}
