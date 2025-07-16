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

    let similariy: u32 = list1
        .into_iter()
        .map(|e1| (list2.iter().filter(|e2| e2.eq(&&e1)).count() as u32) * e1)
        .sum();

    println!("{similariy}");
}
