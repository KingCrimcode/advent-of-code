use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn generate_counts(input: &[Vec<char>]) -> Vec<Vec<i8>> {
    let n = input[0].len();
    let mut roll_count: Vec<Vec<i8>> = vec![vec![8; n]; n];
    (1..n - 1).for_each(|i| {
        roll_count[0][i] = 5;
        roll_count[i][n - 1] = 5;
        roll_count[i][0] = 5;
        roll_count[n - 1][i] = 5;
    });
    roll_count[0][0] = 3;
    roll_count[0][n - 1] = 3;
    roll_count[n - 1][0] = 3;
    roll_count[n - 1][n - 1] = 3;
    roll_count
}

struct FirstRunResult(VecDeque<(usize, usize)>, HashSet<(usize, usize)>);

fn first_run(input: &[Vec<char>], roll_count: &mut [Vec<i8>]) -> FirstRunResult {
    let mut checked = HashSet::new();
    let mut to_mark = VecDeque::new();
    input.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, &c)| {
            if c == '.' {
                DIRECTIONS.iter().for_each(|&(i_change, j_change)| {
                    let (Some(nb_i), Some(nb_j)) = (
                        i.checked_add_signed(i_change),
                        j.checked_add_signed(j_change),
                    ) else {
                        return;
                    };
                    if !(nb_i < input.len() && nb_j < input[0].len()) {
                        return;
                    }
                    roll_count[nb_i][nb_j] -= 1;
                    if input[nb_i][nb_j] == '@'
                        && roll_count[nb_i][nb_j] < 4
                        && checked.insert((nb_i, nb_j))
                    {
                        to_mark.push_back((nb_i, nb_j));
                    }
                })
            }
        })
    });
    FirstRunResult(to_mark, checked)
}

fn solve_part_1(input: &[Vec<char>]) -> usize {
    let mut roll_count = generate_counts(input);
    let FirstRunResult(to_mark, _) = first_run(input, &mut roll_count);
    to_mark.len()
}

fn solve_part_2(input: &mut [Vec<char>]) -> usize {
    let mut roll_count = generate_counts(input);
    let FirstRunResult(mut to_mark, mut checked) = first_run(input, &mut roll_count);
    let mut answer = to_mark.len();
    while let Some((i, j)) = to_mark.pop_front() {
        input[i][j] = 'x';
        DIRECTIONS.iter().for_each(|&(i_change, j_change)| {
            let (Some(nb_i), Some(nb_j)) = (
                i.checked_add_signed(i_change),
                j.checked_add_signed(j_change),
            ) else {
                return;
            };
            if !(nb_i < input.len() && nb_j < input[0].len()) {
                return;
            }
            roll_count[nb_i][nb_j] -= 1;
            if input[nb_i][nb_j] == '@'
                && roll_count[nb_i][nb_j] < 4
                && checked.insert((nb_i, nb_j))
            {
                to_mark.push_back((nb_i, nb_j));
                answer += 1;
            }
        })
    }
    answer
}

fn main() {
    ["example.txt", "input.txt"]
        .into_iter()
        .for_each(|filename| {
            let path = Path::new(filename);
            let file_reader = BufReader::new(File::open(path).expect("file not found"));
            let mut input: Vec<Vec<char>> = file_reader
                .lines()
                .map(|l| l.unwrap().chars().collect::<Vec<char>>())
                .collect();

            println!(
                "{filename}\npart 1: {}\npart 2: {}\n",
                solve_part_1(&input),
                solve_part_2(&mut input)
            )
        });
}
