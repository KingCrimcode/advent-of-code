use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn is_safe(level: &[u32]) -> bool {
    (level.windows(2).all(|w| w[0] <= w[1]) || level.windows(2).all(|w| w[0] >= w[1]))
        && level
            .windows(2)
            .all(|w| w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3)
}

fn main() {
    let path = Path::new("input.txt");
    let file = File::open(path).expect("File not found");
    let file_reader = BufReader::new(file);

    let matrix: Vec<Vec<u32>> = file_reader
        .lines()
        .map(|line| {
            line.expect("File read error")
                .split_whitespace()
                .map(|num| num.parse().expect("Non number found"))
                .collect()
        })
        .collect();

    let safe_count: u32 = matrix
        .iter()
        .filter(|level| {
            is_safe(level)
                || (0..level.len()).any(|i| {
                    let copy: Vec<u32> = level
                        .iter()
                        .enumerate()
                        .filter(|(j, _)| *j != i)
                        .map(|(_, &elem)| elem)
                        .collect();
                    is_safe(&copy)
                })
        })
        .count() as u32;

    println!("{safe_count}")
}
