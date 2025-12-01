use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn function_match(candidate: &str) -> Option<u32> {
    println!("{candidate}");
    let comma = candidate.find(',')?;
    let first = candidate[..comma].trim().parse::<u32>().ok()?;
    let second = candidate[comma + 1..].trim().parse::<u32>().ok()?;
    Some(first * second)
}

fn main() {
    let path = Path::new("example.txt");
    let file_reader = BufReader::new(File::open(path).expect("File not found"));

    let sum: u32 = file_reader
        .lines()
        .map(|line| {
            let mut line_sum = 0;
            let mut remaining_line = line.expect("File read error");
            while let (Some(start), Some(end)) =
                (remaining_line.find("mul("), remaining_line.find(")"))
            {
                if start < end {
                    if let Some(mul) = function_match(&remaining_line[start + 4..end]) {
                        line_sum += mul;
                    }
                    remaining_line = remaining_line.split_off(start + 4);
                }
                else {
                    remaining_line = remaining_line.split_off(end + 1);
                }
            }
            line_sum
        })
        .sum();

    println!("{sum}");
}
