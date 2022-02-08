use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::sync::Mutex;

use combine::Parser;
use counter::Counter;
use itertools::Itertools;
use nalgebra::{IsometryMatrix3, Matrix3, Point3, Rotation3, Translation3};
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

lazy_static::lazy_static! {
    static ref ROTATIONS: [Rotation3<f32>; 24] = itertools::iproduct!(
        [1_f32, -1_f32].into_iter(),
        [1_f32, -1_f32].into_iter(),
        [1_f32, -1_f32].into_iter(),
        [0, 1, 2].into_iter().permutations(3)
    )
    .map(|(sgn1, sgn2, sgn3, ixs)| {
        let mut m = Matrix3::zeros();
        for (i, (sgn, j)) in [sgn1, sgn2, sgn3].iter().copied().zip(ixs).enumerate() {
            m[(i, j)] = sgn;
        }
        m
    })
    .filter(|m| m.determinant() == 1_f32)
    .map(Rotation3::from_matrix_unchecked)
    .collect_vec()
    .try_into()
    .unwrap_or_else(|_| unreachable!());
}

pub fn parse_input<'a>() -> impl Parser<&'a str, Output = Vec<Vec<Point3<i32>>>> {
    use combine::parser::char::*;
    use combine::*;

    fn triple<'a>() -> impl Parser<&'a str, Output = Point3<i32>> {
        use crate::combine_parse_integral;
        combine_parse_integral()
            .skip(char(','))
            .and(combine_parse_integral())
            .skip(char(','))
            .and(combine_parse_integral())
            .map(|((x, y), z)| Point3::new(x, y, z))
    }

    fn scanner<'a>() -> impl Parser<&'a str, Output = Vec<Point3<i32>>> {
        skip_many1(none_of(['\n']))
            .skip(newline())
            .and(sep_end_by1(triple(), newline()))
            .map(|(_, s)| s)
    }

    sep_by1(scanner(), newline()).skip(eof())
}

#[derive(PartialEq, Eq, Clone, Default, Debug)]
struct ScannerFrame {
    beacons: HashSet<Point3<i32>>,
    fingerprint: Counter<i32>,
}

impl ScannerFrame {
    fn from_distances(distances: Vec<Point3<i32>>) -> Self {
        Self {
            beacons: HashSet::from_iter(distances.iter().copied()),
            fingerprint: Counter::from_iter(
                distances
                    .iter()
                    .copied()
                    .cartesian_product(distances.iter().copied())
                    .map(|(u, v)| (u - v).cast::<f32>().norm_squared() as i32),
            ),
        }
    }

    fn make_consistent(&self, other: &ScannerFrame, needed: usize) -> Option<IsometryMatrix3<f32>> {
        if (self.fingerprint.clone() & other.fingerprint.clone()).len() <= needed * (needed - 1) / 2
        {
            return None;
        }
        for (b1, b2) in self.beacons.iter().cartesian_product(other.beacons.iter()) {
            for r in *ROTATIONS {
                let t = Translation3::from(b2.cast() - r * b1.cast());
                let iso = IsometryMatrix3::from_parts(t.cast(), r);
                let beacons = self
                    .beacons
                    .iter()
                    .map(|u| (iso * u.cast()).map(|n| n as i32))
                    .collect::<HashSet<_>>();
                if beacons.intersection(&other.beacons).count() >= needed {
                    return Some(iso);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test_make_consistent {
    use super::*;

    lazy_static::lazy_static! {
        static ref FRAME1: ScannerFrame = ScannerFrame::from_distances(
            vec![
                Point3::new(0, 1, 0),
                Point3::new(1, 2, 0),
                Point3::new(3, 5, 0),
            ]);

        // Just rotate by 90 degrees about the origin.
        static ref FRAME2: ScannerFrame = ScannerFrame::from_distances(
            vec![
                Point3::new(-1, 0, 0),
                Point3::new(-2, 1, 0),
                Point3::new(-5, 3, 0)
            ]);

        // Rotate by 180 degrees and translate.
        static ref FRAME3: ScannerFrame = ScannerFrame::from_distances(
            vec![
                Point3::new(1, 1, 0),
                Point3::new(0, 0, 0),
                Point3::new(-2, -3, 0),
            ]);
    }

    #[test]
    fn test_make_consistent_self() {
        assert!(FRAME1.make_consistent(&FRAME1, 3).is_some());
        assert!(FRAME2.make_consistent(&FRAME2, 3).is_some());
    }

    #[test]
    fn test_make_consistent_rot90() {
        assert!(FRAME1.make_consistent(&FRAME2, 3).is_some());
        assert!(FRAME2.make_consistent(&FRAME1, 3).is_some());
    }

    #[test]
    fn test_make_consistent_rot18() {
        assert!(FRAME1.make_consistent(&FRAME3, 3).is_some());
        assert!(FRAME3.make_consistent(&FRAME1, 3).is_some());
    }
}

pub struct Solution {
    pub scanners: HashSet<Point3<i32>>,
    pub beacons: HashSet<Point3<i32>>,
}

#[derive(Eq, PartialEq, Clone, Default, Debug)]
pub struct Problem {
    scanners: Vec<ScannerFrame>,
}

impl Problem {
    pub fn from_input(input: Vec<Vec<Point3<i32>>>) -> Self {
        Problem {
            scanners: input
                .into_iter()
                .map(ScannerFrame::from_distances)
                .collect_vec(),
        }
    }

    pub fn solve(&self) -> Solution {
        // Make a graph of scanner views, with edges given by
        // transformations that make views consistent.  Specifically,
        // if there is an edge t from s1 to s2, then t(s1) is
        // consistent with s2.  Also, if there is an edge t from s1 to
        // s2, then there is an edge t' from s2 to s1, where t' is the
        // inverse of t.
        //
        // The goal is to form a tree of scanner views. Once we have
        // that, we can walk the tree and convert everything into the
        // frame of the root node.
        let mut graph = Graph::new();
        for i in 0..self.scanners.len() {
            graph.add_node(i);
        }
        let graph = Mutex::new(graph);
        let pairs = self
            .scanners
            .iter()
            .enumerate()
            .cartesian_product(self.scanners.iter().enumerate())
            .filter(|((i, _), (j, _))| i < j)
            .collect_vec();
        pairs.par_iter().for_each(|((i, s1), (j, s2))| {
            if let Some(t) = s1.make_consistent(s2, 12) {
                let mut graph = graph.lock().unwrap();
                graph.add_edge(NodeIndex::new(*i), NodeIndex::new(*j), t);
                graph.add_edge(NodeIndex::new(*j), NodeIndex::new(*i), t.inverse());
            }
        });
        let graph = graph.into_inner().unwrap();
        let mut dfs = petgraph::visit::Dfs::new(&graph, NodeIndex::new(0));
        // Map from nodes to their parents.
        let mut parents = HashMap::new();
        let mut scanners = HashSet::new();
        let mut beacons = HashSet::new();
        let mut isometries = HashMap::new();
        while let Some(node) = dfs.next(&graph) {
            let iso = match parents.get(&node) {
                // Root node.
                None => IsometryMatrix3::identity(),
                // Child of some parent.
                Some(parent) => {
                    isometries[parent]
                        * graph
                            .edge_weight(graph.find_edge(node, *parent).unwrap())
                            .unwrap()
                }
            };
            for other in dfs.stack.iter() {
                if !parents.contains_key(other) {
                    parents.insert(*other, node);
                }
            }
            scanners.insert((iso * Point3::new(0_f32, 0_f32, 0_f32)).map(|n| n as i32));
            beacons.extend(
                self.scanners[node.index()]
                    .beacons
                    .iter()
                    .map(|b| (iso * b.cast()).map(|n| n as i32)),
            );
            isometries.insert(node, iso);
        }
        Solution { scanners, beacons }
    }
}
