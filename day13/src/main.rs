use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::{Hash, Hasher},
    str::FromStr,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part1 = solve_day_13_part1(INPUT);

    println!("part1 -> {}", part1);

    // let part2 = solve_day_13_part2(INPUT);

    // println!("part2 -> {}", part2);
}

fn solve_day_13_part1(input: &str) -> u128 {
    input
        .split("\n\n")
        .map(|s| Pattern::from_str(s).unwrap())
        .map(|p| {
            println!("PATTERN\n{:?}\n\n", p);
            p.find_mirror().unwrap()
        })
        .sum()
}

fn solve_day_13_part2(input: &str) -> u128 {
    0
}

#[cfg(test)]
const EXAMPLE_DATA_1: &str = include_str!("../test1.txt");

#[test]
fn example() {
    let part1 = solve_day_13_part1(EXAMPLE_DATA_1);
    assert_eq!(part1, 405);
}

#[derive(Debug)]
struct Pattern {
    columns: HashMap<String, HashSet<usize>>,
    rows: HashMap<String, HashSet<usize>>,
    col_len: usize,
    row_len: usize,
}

impl Pattern {
    fn transpose(v: &Vec<String>) -> Vec<String> {
        assert!(!v.is_empty());
        let len = v[0].len();
        let mut vv = v.clone();
        let mut iters: Vec<_> = vv.iter_mut().map(|n| n.chars()).collect();
        (0..len)
            .map(|_| {
                iters
                    .iter_mut()
                    .map(|n| n.next().unwrap())
                    .collect::<String>()
            })
            .collect()
    }

    fn find_mirror(&self) -> Option<u128> {
        let r_m = self.find_row_mirror();
        let c_m = self.find_col_mirror();

        match (r_m, c_m) {
            (None, None) => None,
            (None, Some(a)) => Some(a),
            (Some(a), _) => Some(a),
        }
    }

    fn find_row_mirror(&self) -> Option<u128> {
        let mirrors: HashSet<usize> = (1..self.row_len).collect();

        for m in mirrors {
            println!("\nMIRROR ROW {} [{}]", m, self.row_len);
            if self.rows.iter().all(|(_, v)| {
                v.iter().filter(|&&r| r < m).all(|r| {
                    let value = 2 * m - r - 1;
                    println!(
                        "{} {} -> {}",
                        r,
                        value,
                        value >= self.row_len || v.contains(&value)
                    );
                    value >= self.row_len || v.contains(&value)
                })
            }) {
                println!("\nFOUND MIRROR ROW {}\n", m);
                return Some(m as u128 * 100);
            }
        }

        None
    }

    fn find_col_mirror(&self) -> Option<u128> {
        let mirrors: HashSet<usize> = (1..self.col_len).collect();

        for m in mirrors {
            println!("\nMIRROR COL {} [{}]", m, self.col_len);
            if self.columns.iter().all(|(_, v)| {
                v.iter().filter(|&&r| r < m).all(|r| {
                    let value = 2 * m - r - 1;
                    println!(
                        "{} {} -> {}",
                        r,
                        value,
                        value >= self.col_len || v.contains(&value)
                    );
                    value >= self.col_len || v.contains(&value)
                })
            }) {
                println!("\nFOUND MIRROR COL {}\n", m);
                return Some(m as u128);
            }
        }

        None
    }
}

#[derive(Debug)]
struct ParsePatternError {}

impl FromStr for Pattern {
    type Err = ParsePatternError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let rows_str: Vec<String> = input.lines().map(|r| r.trim().to_string()).collect();
        let cols_str = Self::transpose(&rows_str);

        let mut columns = HashMap::<String, HashSet<usize>>::new();
        let mut rows = HashMap::<String, HashSet<usize>>::new();

        for (i, c) in cols_str.iter().enumerate() {
            if columns.get(c).is_none() {
                columns.insert(c.clone(), HashSet::<usize>::new());
            }

            columns.entry(c.clone()).and_modify(|v| {
                v.insert(i);
            });
        }

        for (i, r) in rows_str.iter().enumerate() {
            if rows.get(r).is_none() {
                rows.insert(r.clone(), HashSet::<usize>::new());
            }

            rows.entry(r.clone()).and_modify(|v| {
                v.insert(i);
            });
        }

        Ok(Pattern {
            rows,
            columns,
            col_len: cols_str.len(),
            row_len: rows_str.len(),
        })
    }
}
