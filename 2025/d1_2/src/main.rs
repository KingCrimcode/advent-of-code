use std::{fs::File, io::BufRead, io::BufReader, path::Path};

fn main() {
    // let path = Path::new("example.txt");
    let path = Path::new("input.txt");
    let file_reader = BufReader::new(File::open(path).expect("File not found"));

    let input: Vec<(char, i64)> = file_reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (dir, change) = line.split_at(1);
            let change = change.parse::<i64>().unwrap();
            let dir = dir.chars().next().unwrap();
            (dir, change)
        })
        .collect();

    let mut dial: i64 = 50;
    let mut passwd: i64 = 0;
    input.into_iter().for_each(|(dir, change)| {
        passwd += change / 100;
        let change = change % 100;
        match dir {
            'L' => {
                if dial > change {
                    dial -= change;
                } else {
                    // do not add if dial starts on 0
                    if dial != 0 {
                        passwd += 1;
                    }
                    // mod 100 for the case the dial reaches 0
                    dial = (100 - (change - dial)) % 100;
                }
            }
            'R' => {
                dial += change;
                if dial >= 100 {
                    dial %= 100;
                    passwd += 1;
                }
            }
            _ => (),
        }
    });
    println!("{passwd}");
}
