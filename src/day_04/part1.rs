use aoc2021::day_04::*;
use aoc2021::*;

fn run_game(game: &mut Game) -> (usize, Layout) {
    for (i, n) in game.numbers.iter().enumerate() {
        for board in &mut game.boards {
            if board.mark_and_check(*n) {
                return (i, board.layout.clone());
            }
        }
    }
    panic!("invalid input (no winner)");
}

fn run(input: Input) -> usize {
    let mut game = Game::from_input(input);
    let (winning_index, winning_layout) = run_game(&mut game);
    let called_numbers = &game.numbers[..=winning_index];
    score_layout(&winning_layout, called_numbers)
}

make_main! {4, parse_input, run}
make_test! {04, 1, parse_input, run, 71708}
