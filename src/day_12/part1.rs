use aoc2021::day_12::*;
use aoc2021::*;
use Size::*;

fn step(graph: &Graph, visited: &mut [bool], i: usize) -> u32 {
    if visited[i] {
        return 0;
    }
    if i == graph.end {
        return 1;
    }
    if graph.nodes[i].size == Small {
        visited[i] = true;
    }
    let ret = graph.nodes[i]
        .edges
        .iter()
        .copied()
        .fold_map(|j| step(graph, visited, j));
    visited[i] = false;
    ret
}

fn run(input: Graph) -> u32 {
    let mut visited = vec![false; input.nodes.len()];
    step(&input, &mut visited, input.start)
}

make_main! {12, parse_input, run}
make_test! {12, 1, parse_input, run, 5178}
