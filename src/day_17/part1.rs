use aoc2021::day_17::Input;

fn run(input: Input) -> i32 {
    input.max_apogee()
}

aoc2021::make_main_combine!(17, aoc2021::day_17::parse_input, run);
aoc2021::make_test_combine!(17, 1, aoc2021::day_17::parse_input, run, 33670);
