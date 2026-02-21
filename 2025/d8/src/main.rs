use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use petgraph::{
    algo::{condensation, connected_components, has_path_connecting},
    graph::{NodeIndex, UnGraph},
};

struct Point {
    x: u64,
    y: u64,
    z: u64,
}

struct Distance {
    p1: usize,
    p2: usize,
    dist_squared: u64,
}

impl Distance {
    fn new(idx1: usize, idx2: usize, p1: &Point, p2: &Point) -> Self {
        Distance {
            p1: idx1,
            p2: idx2,
            dist_squared: p1.x.abs_diff(p2.x).pow(2)
                + p1.y.abs_diff(p2.y).pow(2)
                + p1.z.abs_diff(p2.z).pow(2),
        }
    }
}

fn solve_part_1(input: &[Point], checks: usize) -> u64 {
    let mut distances: Vec<Distance> = Vec::new();
    (0..input.len()).for_each(|i| {
        ((i + 1)..input.len()).for_each(|j| {
            distances.push(Distance::new(i, j, &input[i], &input[j]));
        })
    });
    distances.sort_by(|a, b| a.dist_squared.cmp(&b.dist_squared));
    let mut circuits: UnGraph<usize, ()> = Default::default();
    let ciruit_idxs: Vec<NodeIndex<u32>> = (0..input.len()).map(|i| circuits.add_node(i)).collect();
    (0..checks).for_each(|i| {
        if !has_path_connecting(
            &circuits,
            ciruit_idxs[distances[i].p1],
            ciruit_idxs[distances[i].p2],
            None,
        ) {
            circuits.add_edge(
                ciruit_idxs[distances[i].p1],
                ciruit_idxs[distances[i].p2],
                (),
            );
        }
    });
    let condensed_circuits = condensation(circuits, false);
    let circuit_lengths = condensed_circuits
        .node_indices()
        .map(|idx| condensed_circuits[idx].len())
        .collect::<Vec<usize>>();
    let largest_circuit_lengths = circuit_lengths.iter().fold((0, 0, 0), |acc, &len| {
        if len > acc.0 {
            (len, acc.0, acc.1)
        } else if len > acc.1 {
            (acc.0, len, acc.1)
        } else if len > acc.2 {
            (acc.0, acc.1, len)
        } else {
            acc
        }
    });
    (largest_circuit_lengths.0 * largest_circuit_lengths.1 * largest_circuit_lengths.2) as u64
}

fn solve_part_2(input: &[Point]) -> u64 {
    let mut distances: Vec<Distance> = Vec::new();
    (0..input.len()).for_each(|i| {
        ((i + 1)..input.len()).for_each(|j| {
            distances.push(Distance::new(i, j, &input[i], &input[j]));
        })
    });
    distances.sort_by(|a, b| a.dist_squared.cmp(&b.dist_squared));
    let mut circuits: UnGraph<usize, ()> = Default::default();
    let ciruit_idxs: Vec<NodeIndex<u32>> = (0..input.len()).map(|i| circuits.add_node(i)).collect();
    let mut i: usize = 0;
    loop {
        if !has_path_connecting(
            &circuits,
            ciruit_idxs[distances[i].p1],
            ciruit_idxs[distances[i].p2],
            None,
        ) {
            circuits.add_edge(
                ciruit_idxs[distances[i].p1],
                ciruit_idxs[distances[i].p2],
                (),
            );
            if connected_components(&circuits) == 1 {
                return input[distances[i].p1].x * input[distances[i].p2].x;
            }
        }
        i += 1;
    }
}

fn main() {
    [("example.txt", 10), ("input.txt", 1000)]
        .into_iter()
        .for_each(|(filename, checks)| {
            let path = Path::new(filename);
            let file_reader = BufReader::new(File::open(path).expect("file not found"));
            let input: Vec<Point> = file_reader
                .lines()
                .map(|l| {
                    l.unwrap()
                        .split(',')
                        .map(|n| n.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>()
                })
                .map(|v| Point {
                    x: v[0],
                    y: v[1],
                    z: v[2],
                })
                .collect();

            println!(
                "{filename}\npart 1: {}\npart 2: {}\n",
                solve_part_1(&input, checks),
                solve_part_2(&input),
            )
        })
}
