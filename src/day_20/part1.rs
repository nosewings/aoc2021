use aoc2021::day_20::Input;

fn run(mut input: Input) -> usize {
    input.enhance();
    input.enhance();
    input.image.pixels.iter().copied().filter(|x| *x).count()
}

aoc2021::make_main_combine!(20, aoc2021::day_20::parse_input, run);
aoc2021::make_test_combine!(20, 1, aoc2021::day_20::parse_input, run, 5571);
