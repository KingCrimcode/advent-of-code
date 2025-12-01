use std::{fs::File, io::BufRead, io::BufReader, path::Path};

fn main() {
    let path = Path::new("example.txt");
    // let path = Path::new("input.txt");
    let file_reader = BufReader::new(File::open(path).expect("File not found"));

    let input: Vec<(char, u64)> = file_reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (dir, change) = line.split_at(1);
            let change = change.parse::<u64>().unwrap();
            let dir = dir.chars().next().unwrap();
            (dir, change)
        })
        .collect();

    let mut dial: u64 = 50;
    let mut passwd: i64 = 0;
    input.into_iter().for_each(|(dir, change)| {
        let change = change % 100;
        match dir {
            'L' => {
                if dial >= change {
                    dial -= change;
                } else {
                    dial = 100 - (change - dial);
                }
            }
            'R' => dial = (dial + change) % 100,
            _ => (),
        }
        if dial == 0 {
            passwd += 1
        }
    });

    println!("{passwd}");
}
