use aoc2021::day_23::Problem;

fn run(cols: Vec<Vec<u32>>) -> u32 {
    let col0 = vec![cols[0][0], 3, 3, cols[0][1]];
    let col1 = vec![cols[1][0], 1, 2, cols[1][1]];
    let col2 = vec![cols[2][0], 0, 1, cols[2][1]];
    let col3 = vec![cols[3][0], 2, 0, cols[3][1]];
    let cols = vec![col0, col1, col2, col3];
    let problem = Problem::<4, 4, 11>::from_cols(&cols);
    problem.init.solve(&problem.params)
}

aoc2021::make_main_combine_easy!(23, aoc2021::day_23::parse_cols, run);
aoc2021::make_test_combine_easy!(23, 1, aoc2021::day_23::parse_cols, run, 47484);
