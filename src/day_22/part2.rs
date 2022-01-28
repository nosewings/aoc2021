use aoc2021::day_22::Input;

fn run(input: Input) -> i64 {
    input.solve()
}

aoc2021::make_main_combine!(22, aoc2021::day_22::parse_input, run);
aoc2021::make_test_combine!(22, 1, aoc2021::day_22::parse_input, run, 1285501151402480);
