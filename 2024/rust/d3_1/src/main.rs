use std::{
    cmp::min,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn function_match(candidate: &str) -> Option<u32> {
    let comma = candidate.find(',');
    comma?;
    let comma = comma.unwrap();

    match &candidate[..comma].parse::<u32>() {
        Err(_) => None,
        Ok(first) => match &candidate[comma + 1..].parse::<u32>() {
            Err(_) => None,
            Ok(second) => Some(first * second),
        },
    }
}

fn main() {
    let path = Path::new("input.txt");
    let file_reader = BufReader::new(File::open(path).expect("File not found"));

    let sum: u32 = file_reader
        .lines()
        .map(|line| {
            let mut line_sum = 0;
            let mut remaining_line = line.expect("File read error").clone();
            loop {
                let start_index = remaining_line.find("mul(");
                let end_index = remaining_line.find(")");
                if start_index.is_none() || end_index.is_none() {
                    break;
                }
                if start_index < end_index {
                    let mul = function_match(
                        &remaining_line[start_index.unwrap() + 4..end_index.unwrap()],
                    );
                    if mul.is_some() {
                        line_sum += mul.unwrap();
                    }
                }
                remaining_line =
                    remaining_line.split_off(min(start_index.unwrap(), end_index.unwrap()) + 1);
            }
            line_sum
        })
        .sum();

    println!("{sum}");
}
