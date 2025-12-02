use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    // let path = Path::new("example.txt");
    let path = Path::new("input.txt");
    let file_reader = BufReader::new(File::open(path).expect("File not found"));

    let input: Vec<(String, String)> = file_reader
        .lines()
        .flat_map(|line| {
            let line = line.unwrap();
            line.split(',')
                .map(|range| {
                    let (lower, upper) = range.split_once('-').unwrap();
                    (lower.to_string(), upper.to_string())
                })
                .collect::<Vec<(String, String)>>()
        })
        .collect();

    let mut sum: u64 = 0;
    input.into_iter().for_each(|(lower, upper)| {
        let mut id_src = lower.split_at(lower.len() / 2).0.parse::<u64>().unwrap_or(0);
        let id_half = id_src.to_string();
        let id = format!("{}{}", id_half, id_half);

        let mut id_val = id.parse::<u64>().unwrap();
        let lower = lower.parse::<u64>().unwrap();
        let upper = upper.parse::<u64>().unwrap();

        while id_val <= upper {
            if id_val >= lower {
                sum += id_val;
            }

            id_src += 1;
            let id_half = id_src.to_string();
            let id = format!("{}{}", id_half, id_half);
            id_val = id.parse::<u64>().unwrap();
        }
    });

    println!("{sum}");
}
