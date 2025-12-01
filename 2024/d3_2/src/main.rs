use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn function_match(candidate: &str) -> Option<u32> {
    let comma = candidate.find(',')?;
    let first = candidate[..comma].trim().parse::<u32>().ok()?;
    let second = candidate[comma + 1..].trim().parse::<u32>().ok()?;
    Some(first * second)
}

fn main() {
    let path = Path::new("input.txt");
    let file_reader = BufReader::new(File::open(path).expect("File not found"));

    let mut enabled = 1;
    let sum: u32 = file_reader
        .lines()
        .map(|line| {
            let mut line_sum = 0;
            let mut remaining_line = line.expect("File read error");
            while let (Some(start), Some(end)) =
                (remaining_line.find("mul("), remaining_line.find(")"))
            {
                if let Some(i_enable) = remaining_line.find("do()") {
                    if i_enable < start {
                        enabled = 1;
                        remaining_line = remaining_line.split_off(i_enable + 4);
                        continue;
                    }
                }
                if let Some(i_disable) = remaining_line.find("don't()") {
                    if i_disable < start {
                        enabled = 0;
                        remaining_line = remaining_line.split_off(i_disable + 7);
                        continue;
                    }
                }
                if start < end {
                    if let Some(mul) = function_match(&remaining_line[start + 4..end]) {
                        line_sum += mul * enabled;
                    }
                    remaining_line = remaining_line.split_off(start + 4);
                } else {
                    remaining_line = remaining_line.split_off(end + 1);
                }
            }
            line_sum
        })
        .sum();

    println!("{sum}");
}
