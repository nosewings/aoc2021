use aoc2021::day_16::*;
use aoc2021::*;

fn run(input: Packet) -> u64 {
    input.eval()
}

make_main! {16, parse_packet, run}
make_test! {16, 1, parse_packet, run, 1495959086337}
