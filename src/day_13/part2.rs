use aoc2021::day_13::*;
use aoc2021::*;

fn run(mut input: Input) -> String {
    for fold in input.folds {
        fold.execute(&mut input.dots);
    }

    let max_x = input.dots.iter().map(|p| p.0).max().unwrap() as usize;
    let max_y = input.dots.iter().map(|p| p.1).max().unwrap() as usize;
    let mut buf = Vec::new();
    for y in 0..=max_y {
        for x in 0..=max_x {
            buf.push(if input.dots.contains(&(x as u32, y as u32)) {
                '#'
            } else {
                '.'
            });
        }
        buf.push('\n')
    }
    buf.into_iter().collect::<String>()
}

make_main! {13, parse_input, run}
// The output string should say "PCPHARKL".
