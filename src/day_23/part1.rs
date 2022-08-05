use aoc2021::day_23::Problem;

fn run(cols: Vec<Vec<u32>>) -> u32 {
    let problem = Problem::<4, 2, 11>::from_cols(&cols);
    problem.init.solve(&problem.params)
}

aoc2021::make_main_combine_easy!(23, aoc2021::day_23::parse_cols, run);
aoc2021::make_test_combine_easy!(23, 1, aoc2021::day_23::parse_cols, run, 19046);
