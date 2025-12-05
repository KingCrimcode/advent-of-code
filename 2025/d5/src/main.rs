use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn parse_input(input: Vec<String>) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut blank: bool = false;
    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut ids: Vec<u64> = vec![];
    input.into_iter().for_each(|line| {
        if line.is_empty() {
            blank = true;
            return;
        }
        match blank {
            false => {
                let (lower, upper) = line.split_once('-').unwrap();
                let (lower, upper) = (lower.parse::<u64>().unwrap(), upper.parse::<u64>().unwrap());
                ranges.push((lower, upper));
            }
            true => {
                ids.push(line.parse::<u64>().unwrap());
            }
        }
    });
    (ranges, ids)
}

fn solve_part_1((ranges, ids): &(Vec<(u64, u64)>, Vec<u64>)) -> u64 {
    ids.iter().fold(0, |acc, id| {
        match ranges
            .iter()
            .find(|(lower, upper)| lower <= id && id <= upper)
        {
            Some(_) => acc + 1,
            None => acc,
        }
    })
}

fn solve_part_2((ranges, _): &mut (Vec<(u64, u64)>, Vec<u64>)) -> u64 {
    ranges.sort_by(|(a, b), (c, d)| match a.cmp(c) {
        std::cmp::Ordering::Equal => b.cmp(d),
        other => other,
    });
    let mut greatest_checked: u64 = 0;
    ranges.iter().fold(0, |acc, (lower, upper)| {
        match lower.cmp(&greatest_checked) {
            std::cmp::Ordering::Greater => {
                greatest_checked = *upper;
                acc + upper - lower + 1
            }
            _ => match upper.cmp(&greatest_checked) {
                std::cmp::Ordering::Greater => {
                    let temp = acc + upper - greatest_checked;
                    greatest_checked = *upper;
                    temp
                }
                _ => acc,
            },
        }
    })
}

fn main() {
    ["example.txt", "input.txt"]
        .into_iter()
        .for_each(|filename| {
            let path = Path::new(filename);
            let file_reader = BufReader::new(File::open(path).expect("file not found"));
            let input: Vec<String> = file_reader.lines().map(|l| l.unwrap()).collect();
            let mut parsed_input = parse_input(input);

            println!(
                "{filename}\npart 1: {}\npart 2: {}\n",
                solve_part_1(&parsed_input),
                solve_part_2(&mut parsed_input)
            )
        });
}
