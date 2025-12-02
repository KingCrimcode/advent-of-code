use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}, path::Path};

const PRIMES: [usize; 18] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61,
];

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
        let mut found: HashSet<u64> = HashSet::new();
        PRIMES.iter().filter(|&p| *p <= upper.len()).for_each(|p| {
            let mut id_src = lower
                .split_at(lower.len() / p)
                .0
                .parse::<u64>()
                .unwrap_or(0);
            let id_part = id_src.to_string();
            let id = id_part.repeat(*p);

            let mut id_val = id.parse::<u64>().unwrap();
            let lower = lower.parse::<u64>().unwrap();
            let upper = upper.parse::<u64>().unwrap();

            while id_val <= upper {
                if id_val >= lower && found.insert(id_val) {
                    sum += id_val;
                }

                id_src += 1;
                let id_part = id_src.to_string();
                let id = id_part.repeat(*p);
                id_val = id.parse::<u64>().unwrap();
            }
        });
    });

    println!("{sum}");
}
