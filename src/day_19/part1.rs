use aoc2021::day_19::Problem;
use nalgebra::Point3;

fn run(input: Vec<Vec<Point3<i32>>>) -> usize {
    let problem = Problem::from_input(input);
    let solution = problem.solve();
    solution.beacons.len()
}

aoc2021::make_main_combine!(19, aoc2021::day_19::parse_input, run);
aoc2021::make_test_combine!(19, 2, aoc2021::day_19::parse_input, run, 357);
