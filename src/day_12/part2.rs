use aoc2021::day_12::*;
use aoc2021::*;
use Size::*;

fn step(graph: &Graph, visited: &mut [bool], mut flag: bool, i: usize) -> u32 {
    if i == graph.end {
        return 1;
    }
    let node = &graph.nodes[i];
    let v = visited[i];
    if v {
        if i == graph.start || flag {
            return 0;
        }
        flag = true;
    } else if node.size == Small {
        visited[i] = true;
    }
    let ret = node
        .edges
        .iter()
        .copied()
        .fold_map(|j| step(graph, visited, flag, j));
    visited[i] = v;
    ret
}

fn run(input: Graph) -> u32 {
    let mut visited = vec![false; input.nodes.len()];
    step(&input, &mut visited, false, input.start)
}

make_main! {12, parse_input, run}
make_test! {12, 2, parse_input, run, 130094}
