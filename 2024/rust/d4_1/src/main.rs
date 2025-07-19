use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

const WORD: &[char] = &['X', 'M', 'A', 'S'];
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

fn is_word(
    matrix: &[Vec<char>],
    (mut i, mut j): (usize, usize),
    (i_change, j_change): (isize, isize),
    word: &[char],
) -> bool {
    for (poz, word_char) in word.iter().enumerate() {
        match matrix.get(i).and_then(|row| row.get(j)) {
            Some(matrix_char) if matrix_char == word_char => {
                if poz < word.len() - 1 {
                    i = match i.checked_add_signed(i_change) {
                        Some(new_i) => new_i,
                        None => return false,
                    };
                    j = match j.checked_add_signed(j_change) {
                        Some(new_j) => new_j,
                        None => return false,
                    };
                }
            }
            _ => return false,
        }
    }
    true
}

fn main() {
    let path = Path::new("input.txt");
    let file_reader = BufReader::new(File::open(path).expect("File not found"));

    let matrix: Vec<Vec<char>> = file_reader
        .lines()
        .map(|line| line.expect("File reading error").chars().collect())
        .collect();

    let count: u32 = (0..matrix.len())
        .map(|i| -> u32 {
            (0..matrix[i].len())
                .map(|j| -> u32 {
                    DIRECTIONS
                        .iter()
                        .map(|&(i_change, j_change)| is_word(&matrix, (i, j), (i_change, j_change), WORD) as u32)
                        .sum()
                })
                .sum()
        })
        .sum();

    println!("{count}");
}
