use aoc2021::day_16::*;
use aoc2021::*;

fn run(input: Packet) -> u32 {
    input.subpackets().fold_map(|p| p.version as u32)
}

make_main! {16, parse_packet, run}
make_test! {16, 1, parse_packet, run, 938}
