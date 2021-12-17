use aoc2021::day_03::*;
use aoc2021::*;

fn run(input: Input) -> u32
where
{
    let o2 = summarize(input.clone(), |len, count| (2 * count >= len) as u32);
    let co2 = summarize(input, |len, count| (2 * count < len) as u32);
    o2 * co2
}

fn summarize<F>(mut input: Input, f: F) -> u32
where
    F: Fn(usize, usize) -> u32,
{
    for i in (0..input.line_length).rev() {
        if input.lines.len() == 1 {
            break;
        }
        let n = input
            .lines
            .iter()
            .fold_map(|bits| ((bits & (1 << i)) >> i) as usize);
        let target = f(input.lines.len(), n);
        input.lines = input
            .lines
            .into_iter()
            .filter(|bits| ((bits & (1 << i)) >> i) == target)
            .collect();
    }
    if input.lines.len() != 1 {
        panic!("invalid input");
    }
    input.lines[0]
}

make_main! {3, parse_input, run}
make_test! {03, 2, parse_input, run, 482500}
