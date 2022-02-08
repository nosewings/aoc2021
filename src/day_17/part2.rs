use aoc2021::day_17::Input;

fn run(input: Input) -> usize {
    input.valid_velocities().count()
}

aoc2021::make_main_combine!(17, aoc2021::day_17::parse_input, run);
aoc2021::make_test_combine!(17, 2, aoc2021::day_17::parse_input, run, 4903);
