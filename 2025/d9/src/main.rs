use std::{
    cmp::{max, min},
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn solve_part_1(input: &[(usize, usize)]) -> usize {
    (0..input.len())
        .flat_map(|i| {
            (i + 1..input.len()).map(move |j| {
                (input[i].0.abs_diff(input[j].0) + 1) * (input[i].1.abs_diff(input[j].1) + 1)
            })
        })
        .max()
        .unwrap()
}

fn draw_line(tiles: &mut [Vec<u64>], input: &[(usize, usize)], index1: usize, index2: usize) {
    if input[index1].0 == input[index2].0 {
        let i = input[index1].0;
        let j_min = min(input[index1].1, input[index2].1);
        let j_max = max(input[index1].1, input[index2].1);
        tiles[i][j_min..=j_max]
            .iter_mut()
            .for_each(|tile| *tile = 1);
    } else {
        let j = input[index1].1;
        let i_min = min(input[index1].0, input[index2].0);
        let i_max = max(input[index1].0, input[index2].0);
        tiles[i_min..=i_max].iter_mut().for_each(|row| row[j] = 1);
    }
}

fn solve_part_2(input: &[(usize, usize)]) -> usize {
    let mut lines: Vec<usize> = input.iter().map(|(line, _)| *line).collect();
    let mut cols: Vec<usize> = input.iter().map(|(_, col)| *col).collect();

    lines.sort_unstable();
    lines.dedup();
    cols.sort_unstable();
    cols.dedup();

    let lines_map: HashMap<usize, usize> = lines.iter().enumerate().map(|(i, &x)| (x, i)).collect();
    let cols_map: HashMap<usize, usize> = cols.iter().enumerate().map(|(i, &x)| (x, i)).collect();

    let input_compressed: Vec<(usize, usize)> = input
        .iter()
        .map(|(line, col)| (lines_map[line], cols_map[col]))
        .collect();
    let mut tiles_compressed: Vec<Vec<u64>> = vec![vec![0; cols_map.len()]; lines_map.len()];

    // Draw shape outline
    (0..input_compressed.len() - 1).for_each(|k| {
        draw_line(&mut tiles_compressed, &input_compressed, k, k + 1);
    });
    draw_line(
        &mut tiles_compressed,
        &input_compressed,
        input_compressed.len() - 1,
        0,
    );

    // Fill shape
    (0..tiles_compressed.len()).for_each(|i| {
        let mut wall_cols: VecDeque<usize> = tiles_compressed[i]
            .iter()
            .enumerate()
            .filter_map(|(j, &tile)| if tile == 1 { Some(j) } else { None })
            .collect();
        let Some(mut prev) = wall_cols.pop_front() else {
            return;
        };
        while let Some(current) = wall_cols.pop_front() {
            if current - prev > 1 {
                tiles_compressed[i][prev + 1..current]
                    .iter_mut()
                    .for_each(|tile| *tile = 1);
                prev = current;
            } else {
                prev = match wall_cols.pop_front() {
                    Some(col) => col,
                    None => return,
                }
            }
        }
    });

    // Prefix sum
    (0..tiles_compressed.len()).for_each(|i| {
        (0..tiles_compressed[i].len()).for_each(|j| {
            let left: u64 = *tiles_compressed[i]
                .get(j.checked_sub(1).unwrap_or(tiles_compressed[0].len()))
                .unwrap_or(&0);
            let up: u64 = *tiles_compressed
                .get(i.checked_sub(1).unwrap_or(tiles_compressed.len()))
                .and_then(|row| row.get(j))
                .unwrap_or(&0);
            let up_left: u64 = *tiles_compressed
                .get(i.checked_sub(1).unwrap_or(tiles_compressed.len()))
                .and_then(|row| row.get(j.checked_sub(1).unwrap_or(tiles_compressed[0].len())))
                .unwrap_or(&0);
            tiles_compressed[i][j] += left + up - up_left;
        });
    });

    if input.len() == 8 {
        (0..tiles_compressed[0].len()).for_each(|j| {
            (0..tiles_compressed.len()).for_each(|i| {
                print!("{:>3}", tiles_compressed[i][j]);
            });
            println!();
        });
    }

    (0..input.len())
        .filter_map(|s| {
            (s + 1..input.len())
                .filter_map(|t| {
                    let i_min = min(lines_map[&input[s].0], lines_map[&input[t].0]);
                    let j_min = min(cols_map[&input[s].1], cols_map[&input[t].1]);
                    let i_max = max(lines_map[&input[s].0], lines_map[&input[t].0]);
                    let j_max = max(cols_map[&input[s].1], cols_map[&input[t].1]);
                    if input.len() == 8 {
                        println!(
                            "({},{}), ({},{}), est. {}, matrix {}",
                            i_min,
                            j_min,
                            i_max,
                            j_max,
                            ((i_max - i_min + 1) * (j_max - j_min + 1)),
                            tiles_compressed[i_max][j_max]
                                + tiles_compressed
                                    .get(i_min.checked_sub(1).unwrap_or(tiles_compressed.len()))
                                    .and_then(|row| {
                                        row.get(
                                            j_min
                                                .checked_sub(1)
                                                .unwrap_or(tiles_compressed[0].len()),
                                        )
                                    })
                                    .unwrap_or(&0)
                                - tiles_compressed[i_max]
                                    .get(j_min.checked_sub(1).unwrap_or(tiles_compressed[0].len()))
                                    .unwrap_or(&0)
                                - tiles_compressed
                                    .get(i_min.checked_sub(1).unwrap_or(tiles_compressed.len()))
                                    .and_then(|row| row.get(j_max))
                                    .unwrap_or(&0)
                        );
                    }
                    if ((i_max - i_min + 1) * (j_max - j_min + 1)) as u64
                        == tiles_compressed[i_max][j_max]
                            + tiles_compressed
                                .get(i_min.checked_sub(1).unwrap_or(tiles_compressed.len()))
                                .and_then(|row| {
                                    row.get(
                                        j_min.checked_sub(1).unwrap_or(tiles_compressed[0].len()),
                                    )
                                })
                                .unwrap_or(&0)
                            - tiles_compressed[i_max]
                                .get(j_min.checked_sub(1).unwrap_or(tiles_compressed[0].len()))
                                .unwrap_or(&0)
                            - tiles_compressed
                                .get(i_min.checked_sub(1).unwrap_or(tiles_compressed.len()))
                                .and_then(|row| row.get(j_max))
                                .unwrap_or(&0)
                    {
                        Some((input[s].0.abs_diff(input[t].0) + 1) * (input[s].1.abs_diff(input[t].1) + 1))
                    } else {
                        None
                    }
                })
                .max()
        })
        .max()
        .unwrap_or(0)
}

fn main() {
    ["example.txt", "input.txt"]
        .into_iter()
        .for_each(|filename| {
            let path = Path::new(filename);
            let file_reader = BufReader::new(File::open(path).expect("file not found"));
            let input: Vec<(usize, usize)> = file_reader
                .lines()
                .map(|l| {
                    l.unwrap()
                        .split_once(',')
                        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                        .unwrap()
                })
                .collect();

            println!(
                "{filename}\npart 1: {}\npart 2: {}\n",
                solve_part_1(&input),
                solve_part_2(&input)
            );
        });
}
