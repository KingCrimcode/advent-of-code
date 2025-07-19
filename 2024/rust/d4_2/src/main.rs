use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

const CONFIG_LIST: [[(isize, isize, char); 4]; 4] = [
    [(-1, -1, 'M'), (-1, 1, 'M'), (1, 1, 'S'), (1, -1, 'S')],
    [(-1, -1, 'S'), (-1, 1, 'M'), (1, 1, 'M'), (1, -1, 'S')],
    [(-1, -1, 'S'), (-1, 1, 'S'), (1, 1, 'M'), (1, -1, 'M')],
    [(-1, -1, 'M'), (-1, 1, 'S'), (1, 1, 'S'), (1, -1, 'M')],
];

fn is_x_mas(
    matrix: &[Vec<char>],
    (i, j): (usize, usize),
    config: [(isize, isize, char); 4],
) -> bool {
    config.iter().all(|&(di, dj, x_char)| {
        i.checked_add_signed(di)
            .zip(j.checked_add_signed(dj))
            .and_then(|(x_i, x_j)| matrix.get(x_i).and_then(|row| row.get(x_j)))
            .is_some_and(|&matrix_char| matrix_char == x_char)
    })
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
                .filter(|&j| matrix[i][j] == 'A')
                .map(|j| -> u32 {
                    CONFIG_LIST
                        .iter()
                        .map(|&config| is_x_mas(&matrix, (i, j), config) as u32)
                        .sum()
                })
                .sum()
        })
        .sum();

    println!("{count}");
}
