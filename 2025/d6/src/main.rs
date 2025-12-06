use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Default)]
struct Problem {
    numbers: Vec<u64>,
    sign: char,
}

fn parse_input(input: &[String]) -> Vec<Problem> {
    let mut parsed_input: Vec<Problem> = vec![];
    input.iter().for_each(|line| {
        let mut i: usize = 0;
        line.split(' ').for_each(|elem| {
            if elem.is_empty() {
                return;
            }
            if parsed_input.len() == i {
                parsed_input.push(Problem::default());
            }
            match elem.parse::<u64>() {
                Ok(num) => parsed_input[i].numbers.push(num),
                Err(_) => parsed_input[i].sign = elem.chars().next().unwrap(),
            }
            i += 1;
        });
    });
    parsed_input
}

fn solve_part_1(input: &[Problem]) -> u64 {
    input
        .iter()
        .map(|problem| match problem.sign {
            '+' => problem.numbers.iter().sum::<u64>(),
            '*' => problem.numbers.iter().product::<u64>(),
            _ => panic!("unknown sign {}", problem.sign),
        })
        .sum()
}

fn solve_part_2(input: &[String]) -> u64 {
    let input = input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut answer: u64 = 0;
    let mut problem: Problem = Problem::default();
    for j in 0..input[0].len() {
        let mut num: String = String::default();
        (0..(input.len() - 1)).for_each(|i| {
            if input[i][j] != ' ' {
                num.push(input[i][j]);
            }
        });
        if input[input.len() - 1][j] != ' ' {
            problem.sign = input[input.len() - 1][j];
        }
        if num.is_empty() {
            answer += match problem.sign {
                '+' => problem.numbers.iter().sum::<u64>(),
                '*' => problem.numbers.iter().product::<u64>(),
                _ => panic!("unknown sign {}", problem.sign),
            };
            problem = Problem::default();
        } else {
            problem.numbers.push(num.parse::<u64>().unwrap());
        }
    }
    answer += match problem.sign {
        '+' => problem.numbers.iter().sum::<u64>(),
        '*' => problem.numbers.iter().product::<u64>(),
        _ => panic!("unknown sign {}", problem.sign),
    };
    answer
}

fn main() {
    ["example.txt", "input.txt"]
        .into_iter()
        .for_each(|filename| {
            let path = Path::new(filename);
            let file_reader = BufReader::new(File::open(path).expect("file not found"));
            let input: Vec<String> = file_reader.lines().map(|l| l.unwrap()).collect();
            let parsed_input = parse_input(&input);

            println!(
                "{filename}\npart 1: {}\npart 2: {}\n",
                solve_part_1(&parsed_input),
                solve_part_2(&input)
            )
        });
}
