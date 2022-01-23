use aoc2021::day_21::{parse_input, Input, Player};

pub struct Game {
    pub players: [Player; 2],
    pub die: u32,
    pub counter: u32,
}

impl Game {
    fn roll(&mut self) -> u32 {
        let ret = self.die;
        self.die = (self.die % 100) + 1;
        self.counter += 1;
        ret
    }

    fn turn(&mut self, n: usize) -> bool {
        let total = (1..=3).map(|_| self.roll()).sum();
        self.players[n].advance(total)
    }

    fn round(&mut self) -> bool {
        self.turn(0) || self.turn(1)
    }

    pub fn play(&mut self) {
        while !self.round() {}
    }
}

impl From<Input> for Game {
    fn from(input: Input) -> Self {
        Self {
            players: input
                .positions
                .map(|position| Player { position, score: 0 }),
            die: 1,
            counter: 0,
        }
    }
}

fn run(input: Input) -> u32 {
    let mut game = Game::from(input);
    game.play();
    let min = game.players.iter().map(|p| p.score).min().unwrap();
    min * game.counter
}

aoc2021::make_main_combine!(21, parse_input, run);
aoc2021::make_test_combine!(21, 1, parse_input, run, 897798);
