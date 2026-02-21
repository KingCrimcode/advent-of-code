use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn solve_part_1(input: &[Vec<char>]) -> u64 {
    let mut flow: Vec<Vec<bool>> = vec![vec![false; input[0].len()]; input.len()];
    let mut queue: VecDeque<(usize, usize)> = Default::default();
    let mut sum: u64 = 0;
    queue.push_back((
        1,
        input[0]
            .iter()
            .enumerate()
            .find(|&(_, &c)| c == 'S')
            .unwrap()
            .0,
    ));
    flow[0][queue.front().unwrap().1] = true;
    while let Some(coords) = queue.pop_front() {
        let Some(current_row) = flow.get_mut(coords.0) else {
            continue;
        };
        let Some(current_cell) = current_row.get_mut(coords.1) else {
            continue;
        };
        if *current_cell {
            continue;
        }
        *current_cell = true;
        match input[coords.0][coords.1] {
            '.' => {
                queue.push_back((coords.0 + 1, coords.1));
            }
            '^' => {
                queue.push_back((coords.0, coords.1 - 1));
                queue.push_back((coords.0, coords.1 + 1));
                sum += 1;
            }
            _ => panic!("unknown character at {}:{}", coords.0 + 1, coords.1 + 1),
        }
    }
    sum
}

fn solve_part_2(input: &mut [Vec<char>]) -> u64 {
    let mut flow: Vec<Vec<u64>> = vec![vec![0; input[0].len()]; input.len()];
    let mut queue: VecDeque<(usize, usize)> = Default::default();
    let start = (
        0,
        input[0]
            .iter()
            .enumerate()
            .find(|&(_, &c)| c == 'S')
            .unwrap()
            .0,
    );
    queue.push_back((start.0 + 1, start.1));
    while let Some(coords) = queue.pop_front() {
        let Some(current_row) = input.get_mut(coords.0) else {
            continue;
        };
        let Some(current_cell) = current_row.get_mut(coords.1) else {
            continue;
        };
        if *current_cell == '|' {
            continue;
        }
        match input[coords.0][coords.1] {
            '.' => {
                input[coords.0][coords.1] = '|';
                queue.push_back((coords.0 + 1, coords.1));
            }
            '^' => {
                queue.push_back((coords.0, coords.1 - 1));
                queue.push_back((coords.0, coords.1 + 1));
            }
            _ => panic!("unknown character at {}:{}", coords.0 + 1, coords.1 + 1),
        }
    }
    for j in 0..input[0].len() {
        if input[input.len() - 1][j] == '|' {
            flow[input.len() - 1][j] = 1;
        }
    }
    for i in (1..(input.len() - 1)).rev() {
        for j in 0..input[0].len() {
            if input[i][j] == '|' {
                flow[i][j] = flow[i + 1][j];
            }
        }
        for j in 0..input[0].len() {
            if input[i][j] == '^' {
                flow[i][j] = flow[i][j - 1] + flow[i][j + 1];
            }
        }
    }
    flow[start.0 + 1][start.1]
}

fn main() {
    ["example.txt", "input.txt"]
        .into_iter()
        .for_each(|filename| {
            let path = Path::new(filename);
            let file_reader = BufReader::new(File::open(path).expect("file not found"));
            let mut input: Vec<Vec<char>> = file_reader
                .lines()
                .map(|l| l.unwrap().chars().collect())
                .collect();

            println!(
                "{filename}\npart 1: {}\npart 2: {}\n",
                solve_part_1(&input),
                solve_part_2(&mut input),
            )
        });
}
