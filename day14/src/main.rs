use std::{collections::HashMap, str::FromStr, time::Instant};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now_1 = Instant::now();
    let part1 = solve_day_14_part1(INPUT);

    println!("part1 -> {}", part1);

    let elapsed_1 = now_1.elapsed();
    println!("Part 1 - Elapsed: {:.2?}", elapsed_1);
    let now_2 = Instant::now();
    let part2 = solve_day_14_part2(INPUT);

    println!("part2 -> {}", part2);

    let elapsed_2 = now_2.elapsed();
    println!("Part2 - Elapsed: {:.2?}", elapsed_2);
}

fn compute_weight(m: &String) -> u64 {
    let len = m.lines().count();
    m.lines()
        .enumerate()
        .map(|(j, r)| {
            let sum = r
                .chars()
                .map(|c| match c {
                    'O' => (len - j) as u64,
                    _ => 0,
                })
                .sum::<u64>();
            sum
        })
        .sum::<u64>()
}

fn solve_day_14_part2(input: &str) -> u64 {
    let mut m: Matrix = Matrix::from_str(input).unwrap();
    // println!("original\n{}", Into::<String>::into(&m));

    let mut res = HashMap::<String, Vec<u32>>::new();

    for i in 0..1000 {
        let k = Into::<String>::into(&m);

        res.entry(k).or_insert(Vec::new()).push(i);
        m.cycle();
    }

    let offset = res
        .clone()
        .values()
        .filter(|r| r.len() == 1)
        .map(|v| v[0])
        .max()
        .unwrap();
    let diff_v = res
        .values()
        .filter(|r| r.len() > 1)
        .collect::<Vec<&Vec<u32>>>();

    let diff_v = diff_v.first().unwrap();

    // for index in offset..(offset + (diff_v[1] - diff_v[0])) {
    //     let matrix = res.iter().find(|(_, v)| v.contains(&index)).unwrap().0;
    //     let x = compute_weight(matrix);
    //     println!("\n{}", matrix);
    //     println!("{} -> {}", index, x);
    // }

    let index = (1000000000 - offset) % (diff_v[1] - diff_v[0]) + offset;

    let matrix = res.iter().find(|(_, v)| v.contains(&index)).unwrap().0;

    compute_weight(matrix)
}

fn solve_day_14_part1(input: &str) -> u64 {
    // println!("{}", input);
    let input_matrix: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.trim().chars().collect::<Vec<char>>())
        .collect();

    let mut next_pos_index = vec![0; input_matrix[0].len()];
    let len = input_matrix.len();

    input_matrix
        .iter()
        .enumerate()
        .map(|(j, r)| {
            // println!("\n{:?}", r);
            // println!("{:?}", next_pos_index);
            let sum = r
                .iter()
                .enumerate()
                .map(|(i, c)| match c {
                    'O' => {
                        next_pos_index[i] += 1;
                        // println!("{}: {} {} -> {} ", i, c, len, (len - next_pos_index[i] + 1));
                        (len - next_pos_index[i] + 1) as u64
                    }
                    '#' => {
                        next_pos_index[i] = j + 1;
                        0
                    }
                    _ => 0,
                })
                .sum::<u64>();
            // println!("{:?}", next_pos_index);
            // println!("{}", sum);
            sum
        })
        .sum::<u64>()
}

struct Matrix {
    levels: Vec<Vec<char>>, // lines
}

impl Matrix {
    fn roll_left(&mut self) {
        for l in 0..self.levels.len() {
            let mut next_index = 0;
            for pos in 0..self.levels[l].len() {
                match self.levels[l][pos] {
                    'O' => {
                        self.levels[l][next_index] = 'O';
                        if next_index != pos {
                            self.levels[l][pos] = '.';
                        }
                        next_index += 1;
                    }
                    '#' => {
                        next_index = pos + 1;
                    }
                    _ => {}
                };
            }
        }
    }

    fn roll_right(&mut self) {
        for l in 0..self.levels.len() {
            let mut next_index = self.levels[l].len() - 1;
            for pos in 0..self.levels[l].len() {
                let real_pos = self.levels[l].len() - 1 - pos;
                match self.levels[l][real_pos] {
                    'O' => {
                        self.levels[l][next_index] = 'O';
                        if next_index != real_pos {
                            self.levels[l][real_pos] = '.';
                        }
                        if (next_index as i32 - 1) >= 0 {
                            next_index -= 1;
                        } else {
                            break;
                        }
                    }
                    '#' => {
                        if (real_pos as i32 - 1) >= 0 {
                            next_index = real_pos - 1;
                        } else {
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn cycle(&mut self) {
        self.levels = transpose(&self.levels);
        self.roll_left();
        self.levels = transpose(&self.levels);
        self.roll_left();
        self.levels = transpose(&self.levels);
        self.roll_right();
        self.levels = transpose(&self.levels);
        self.roll_right();
    }
}

impl Into<String> for &Matrix {
    fn into(self) -> String {
        self.levels
            .iter()
            .map(|vec_char| format!("{}\n", vec_char.iter().collect::<String>()))
            .collect()
    }
}

#[derive(Debug)]
struct ParseError {}

impl FromStr for Matrix {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Vec<Vec<char>> = s
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect();
        Ok(Matrix { levels: res })
    }
}

fn transpose(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut vv = v.clone();
    let mut iters: Vec<_> = vv.iter_mut().map(|n| n.iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap().clone())
                .collect::<Vec<char>>()
        })
        .collect()
}

#[cfg(test)]
const EXAMPLE_DATA_1: &str = include_str!("../test1.txt");

#[test]
fn example() {
    let part1 = solve_day_14_part1(EXAMPLE_DATA_1);
    assert_eq!(part1, 136);

    let part2 = solve_day_14_part2(EXAMPLE_DATA_1);
    assert_eq!(part2, 64);
}
