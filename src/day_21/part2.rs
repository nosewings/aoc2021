#[macro_use]
extern crate itertools;

use aoc2021::day_21::Input;
use ndarray::Array5;

struct Memo {
    inner: Array5<Option<(usize, usize)>>,
}

impl Memo {
    fn new() -> Self {
        Self {
            inner: Array5::from_elem([10, 21, 10, 21, 2], None),
        }
    }

    fn get(
        &mut self,
        pos1: usize,
        score1: usize,
        pos2: usize,
        score2: usize,
        turn: usize,
    ) -> (usize, usize) {
        let ix = (pos1 - 1, score1, pos2 - 1, score2, turn);
        let ret = self.inner[ix].unwrap_or_else(|| {
            itertools::iproduct!(1..=3, 1..=3, 1..=3)
                .map(|(d1, d2, d3)| {
                    let n = d1 + d2 + d3;
                    if turn == 0 {
                        let pos1 = (((pos1 - 1) + n) % 10) + 1;
                        let score1 = score1 + pos1;
                        if score1 >= 21 {
                            (1, 0)
                        } else {
                            self.get(pos1, score1, pos2, score2, 1)
                        }
                    } else {
                        let pos2 = (((pos2 - 1) + n) % 10) + 1;
                        let score2 = score2 + pos2;
                        if score2 >= 21 {
                            (0, 1)
                        } else {
                            self.get(pos1, score1, pos2, score2, 0)
                        }
                    }
                })
                .fold((0, 0), |(a1, b1), (a2, b2)| (a1 + a2, b1 + b2))
        });
        self.inner[ix] = Some(ret);
        ret
    }
}

fn run(input: Input) -> usize {
    let mut memo = Memo::new();
    let (score1, score2) = memo.get(
        input.positions[0] as usize,
        0,
        input.positions[1] as usize,
        0,
        0,
    );
    score1.max(score2)
}

aoc2021::make_main_combine!(21, aoc2021::day_21::parse_input, run);
aoc2021::make_test_combine!(21, 2, aoc2021::day_21::parse_input, run, 48868319769358);
