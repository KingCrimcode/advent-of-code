use std::cmp::min;
use std::io::BufRead;
use std::{fs::File, io::BufReader, path::Path};

const BATTERY_COUNT: usize = 12;

fn solve_part_1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|row| {
            let mut max1: char = '0';
            let mut max2: char = '0';
            row.chars().enumerate().for_each(|(i, c)| {
                if i != row.len() - 1 {
                    if c > max1 {
                        max1 = c;
                        max2 = '0';
                    } else if c > max2 {
                        max2 = c
                    }
                } else if c > max2 {
                    max2 = c
                }
            });
            format!("{max1}{max2}").parse::<u32>().unwrap()
        })
        .sum::<u32>()
}

fn solve_part_2(input: &[String]) -> u64 {
    input
        .iter()
        .map(|row| {
            let mut num: [char; BATTERY_COUNT] = ['0'; BATTERY_COUNT];
            row.chars().enumerate().for_each(|(i, c)| {
                for j in BATTERY_COUNT - min(BATTERY_COUNT, row.len() - i)..BATTERY_COUNT {
                    if c > num[j] {
                        num[j] = c;
                        (j + 1..BATTERY_COUNT).for_each(|k| {
                            num[k] = '0';
                        });
                        break;
                    }
                }
            });
            num.into_iter().collect::<String>().parse::<u64>().unwrap()
        })
        .sum::<u64>()
}

fn main() {
    ["example.txt", "input.txt"]
        .into_iter()
        .for_each(|filename| {
            let path = Path::new(filename);
            let file_reader = BufReader::new(File::open(path).expect("file not found"));
            let input: Vec<String> = file_reader.lines().map(|l| l.unwrap()).collect();

            println!(
                "{filename}\npart 1: {}\npart 2: {}\n",
                solve_part_1(&input),
                solve_part_2(&input)
            );
        })
}
