use aoc2021::day_03::*;
use aoc2021::*;

fn run(input: Input) -> u32 {
    let mut counts = vec![0; input.line_length];
    for bits in &input.lines {
        for (i, count) in counts.iter_mut().enumerate() {
            *count += ((bits & (1 << i)) >> i) as usize;
        }
    }
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for (i, n) in counts.iter().enumerate() {
        let b = (*n as usize) * 2 >= input.lines.len();
        gamma |= (b as u32) << i;
        epsilon |= (!b as u32) << i;
    }
    gamma * epsilon
}

make_main! {3, parse_input, run}
make_test! {03, 1, parse_input, run, 1307354}
