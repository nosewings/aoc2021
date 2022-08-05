use aoc2021::day_04::*;
use aoc2021::*;

fn run_game(game: &mut Game) -> (usize, Layout) {
    let it = &mut game.numbers.iter().enumerate();
    let loser = loop {
        let (_, n) = it.next().expect("invalid input (no single loser)");
        game.boards.retain_mut(|board| !board.mark_and_check(*n));
        if game.boards.len() == 1 {
            break &mut game.boards[0];
        }
    };
    for (i, n) in it {
        if loser.mark_and_check(*n) {
            return (i, loser.layout.clone());
        }
    }
    panic!("invalid input (last board never wins)");
}

fn run(input: Input) -> usize {
    let mut game = Game::from_input(input);
    let (i, layout) = run_game(&mut game);
    score_layout(&layout, &game.numbers[..=i])
}

make_main! {4, parse_input, run}
make_test! {04, 2, parse_input, run, 34726}
