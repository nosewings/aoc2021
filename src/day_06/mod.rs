use ndarray::Array2;
use nom::character::complete::char;
use nom::multi::separated_list1;

use crate::*;

pub fn parse_input<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<u64>> {
    all_consuming(newline_terminated(separated_list1(
        char(','),
        parse_integral_nonnegative(),
    )))
}

fn update_table(table: &mut Array2<Option<u64>>, total: usize, time: usize, timer: usize) -> u64 {
    match table[(time, timer)] {
        Some(n) => n,
        None => {
            let start = time + timer + 1;
            let result = 1
                + (start..=total)
                    .step_by(7)
                    .fold_map(|time| update_table(table, total, time, 8));
            table[(time, timer)] = Some(result);
            result
        }
    }
}

pub fn run_for(input: Vec<u64>, days: usize) -> u64 {
    let mut table = Array2::from_elem((days + 1, 9), None);
    input
        .iter()
        .fold_map(|timer| update_table(&mut table, days, 0, *timer as usize))
}
