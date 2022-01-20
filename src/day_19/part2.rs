use aoc2021::day_19::Problem;
use itertools::Itertools;
use nalgebra::Point3;

fn run(input: Vec<Vec<Point3<i32>>>) -> i32 {
    let problem = Problem::from_input(input);
    let solution = problem.solve();
    solution
        .scanners
        .iter()
        .cartesian_product(solution.scanners.iter())
        .map(|(u, v)| (*u - *v).abs().sum())
        .max()
        .unwrap()
}

aoc2021::make_main_combine!(19, aoc2021::day_19::parse_input, run);
aoc2021::make_test_combine!(19, 2, aoc2021::day_19::parse_input, run, 12317);
