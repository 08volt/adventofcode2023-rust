use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part1 = solve_day_11(INPUT);

    println!("{}", part1);
}

fn solve_day_11(input: &str) -> u128 {
    let cols = input.lines().map(|l| l.trim().len()).max().unwrap_or(0);
    let rows = input.lines().count();

    let mut cols_vec = vec![0; cols];
    let mut rows_vec = vec![0; rows];

    let mut galaxies_pos = Vec::<(usize, usize)>::new();

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.trim().chars().enumerate() {
            if c == '#' {
                cols_vec[x] += 1;
                rows_vec[y] += 1;
                galaxies_pos.push((x, y));
            }
        }
    }
    println!("cols_vec: {:?}\n", cols_vec);
    println!("rows_vec: {:?}\n", rows_vec);

    println!("galaxies_pos: {:?}\n", galaxies_pos);

    let empty_cols: HashSet<usize> = cols_vec
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == 0)
        .map(|(i, _)| i)
        .collect();
    println!("e_cols: {:?}\n", empty_cols);
    let empty_rows: HashSet<usize> = rows_vec
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == 0)
        .map(|(i, _)| i)
        .collect();

    println!("e_rows: {:?}\n", empty_rows);

    let galaxies_couples: HashSet<(usize, usize)> = galaxies_pos
        .iter()
        .enumerate()
        .flat_map(|(i, _)| {
            let couples: Vec<(usize, usize)> =
                (i + 1..galaxies_pos.len()).map(|j| (i, j)).collect();
            couples
        })
        .collect();

    let col_distances: u128 = galaxies_couples
        .iter()
        .map(|(i, j)| {
            let mut i_col = galaxies_pos[i.clone()].0;
            let mut j_col = galaxies_pos[j.clone()].0;

            if i_col > j_col {
                let tmp = j_col;
                j_col = i_col;
                i_col = tmp;
            }

            let double_cols = empty_cols
                .iter()
                .filter(|&c| c > &i_col && c < &j_col)
                .count();

            (j_col - i_col) as u128 + (double_cols as u128) * (999999)
        })
        .sum();

    let row_distances: u128 = galaxies_couples
        .iter()
        .map(|(i, j)| {
            let mut i_row = galaxies_pos[i.clone()].1;
            let mut j_row = galaxies_pos[j.clone()].1;

            if i_row > j_row {
                let tmp = j_row;
                j_row = i_row;
                i_row = tmp;
            }

            let double_rows = empty_rows
                .iter()
                .filter(|&c| c > &i_row && c < &j_row)
                .count();

            (j_row - i_row) as u128 + (double_rows as u128) * (999999)
        })
        .sum();

    (row_distances + col_distances) as u128
}

#[cfg(test)]
const EXAMPLE_DATA_1: &str = include_str!("../test1.txt");
// #[cfg(test)]
// const EXAMPLE_DATA_2: &str = include_str!("../test2.txt");
// #[cfg(test)]
// const EXAMPLE_DATA_3: &str = include_str!("../test3.txt");
// #[cfg(test)]
// const EXAMPLE_DATA_4: &str = include_str!("../test4.txt");
// #[cfg(test)]
// const EXAMPLE_DATA_5: &str = include_str!("../test5.txt");

#[test]
fn example() {
    let part1 = solve_day_11(EXAMPLE_DATA_1);
    assert_eq!(part1, 8410);
    // let part1 = solve_day_10(EXAMPLE_DATA_2);
    // assert_eq!(part1, 8);

    // let part2 = solve_day_10_part2(EXAMPLE_DATA_3);
    // assert_eq!(part2, 4);

    // let part2 = solve_day_10_part2(EXAMPLE_DATA_4);
    // assert_eq!(part2, 10);

    // let part2 = solve_day_10_part2(EXAMPLE_DATA_5);
    // assert_eq!(part2, 8);
}
