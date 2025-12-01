use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let file = File::open(path).expect("File open error");
    let file_reader = BufReader::new(file);
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();

    for line in file_reader.lines() {
        let string = line.unwrap_or_else(|why| panic!("{why:?}"));
        let parts: Vec<&str> = string.split_whitespace().collect();
        if let [ex1, ex2] = parts[..] {
            list1.push(ex1.parse().expect("Non number found"));
            list2.push(ex2.parse().expect("Non number found"));
        } else {
            panic!("Wrong file format");
        }
    }

    list1.sort();
    list2.sort();

    let diff: u32 = list1
        .into_iter()
        .zip(list2)
        .map(|(e1, e2)| e1.abs_diff(e2))
        .sum();

    println!("{diff}");
}
