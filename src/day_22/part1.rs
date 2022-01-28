use aoc2021::day_22::cuboid::Cuboid;
use aoc2021::day_22::vector::Vector;
use aoc2021::day_22::Input;

fn run(mut input: Input) -> i64 {
    let lo = Vector::repeat(-50);
    let hi = Vector::repeat(50);
    let bound = Cuboid::new(lo, hi).unwrap_or_else(|| unreachable!());
    input.steps.retain(|step| step.region.is_sub_cuboid(bound));
    input.solve()
}

aoc2021::make_main_combine!(22, aoc2021::day_22::parse_input, run);
aoc2021::make_test_combine!(22, 1, aoc2021::day_22::parse_input, run, 543306);
