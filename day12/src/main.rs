use itertools::Itertools;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    iter::{repeat, zip},
    str::FromStr,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part1 = solve_day_12(INPUT);

    println!("part1 -> {}", part1);

    let part2 = solve_day12_part2_dp(INPUT);

    println!("part2 -> {}", part2);
}

fn solve_day_12(input: &str) -> u128 {
    input
        .lines()
        .map(|l| {
            let a = Arrangement::from_str(l.trim()).unwrap();
            Arrangement::is_valid(&a.groups, 0, &a.springs, 0)
        })
        .sum()
}

fn _solve_day_12_part2(input: &str) -> u128 {
    let input: Vec<Arrangement> = input
        .lines()
        .map(|l| {
            let arr = Arrangement::from_str(l.trim()).unwrap();
            let mut s = arr.springs;
            s.push(None);
            let mut new_s: Vec<Option<bool>> = repeat(s.clone()).take(5).flatten().collect();
            new_s.pop();

            let new_g: Vec<u128> = repeat(arr.groups).take(5).flatten().collect();

            Arrangement {
                springs: new_s,
                groups: new_g,
            }
        })
        .collect();

    input
        .par_iter()
        .enumerate()
        .map(|(i, a)| {
            // println!("{} \n{:?}\n{:?}\n\n", i, a.springs, a.groups);
            Arrangement::is_valid(&a.groups, 0, &a.springs, 0)
        })
        .sum()
}

fn solve_day12_part2_dp(input: &str) -> u128 {
    let p_lines: Vec<String> = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(' ').collect();
            let groups: Vec<usize> = parts[1]
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            let new_g: Vec<usize> = repeat(groups).take(5).flatten().collect();
            format!(
                "*{}*",
                new_g
                    .iter()
                    .map(|g| "#".repeat(g.clone()))
                    .collect::<Vec<String>>()
                    .join("+")
            )
        })
        .collect();

    zip(input.lines(), p_lines)
        .map(|(l, p)| {
            let parts: Vec<&str> = l.split(' ').collect();

            let mut chrs: String = parts[0].chars().collect();
            chrs.push('?');
            let repeated_chrs = chrs.repeat(5);
            let mut removed_chrs = repeated_chrs.chars();

            removed_chrs.next_back();
            let s: Vec<char> = removed_chrs.collect();
            let s_string: String = s.clone().iter().collect();
            // println!("{:?}\n{:?}\n\n", s_string, p);
            Arrangement::solve(
                0,
                0,
                &s,
                &p.chars().collect_vec(),
                HashMap::<(usize, usize, String), u128>::new(),
            )
            .0
        })
        .sum()
}

#[derive(Debug)]
struct Arrangement {
    groups: Vec<u128>,
    springs: Vec<Option<bool>>,
}

impl Arrangement {
    fn is_valid(
        groups: &Vec<u128>,
        group_index: usize,
        springs: &Vec<Option<bool>>,
        spring_index: usize,
    ) -> u128 {
        if springs.len() <= spring_index {
            let res = groups.len() == group_index;
            // println!("\n1 {}\n", res);
            return res as u128;
        }
        if groups.len() == group_index {
            let res = springs[spring_index..].iter().all(|x| x.unwrap_or(true));
            // println!("\n2 {}\n", res);
            return res as u128;
        }

        let first_spring = springs[spring_index];

        if let Some(true) = first_spring {
            return Self::is_valid(groups, group_index, springs, spring_index + 1);
        }
        let mut working = 0;

        if first_spring == None {
            // if ? try with pipe working
            working = Self::is_valid(groups, group_index, springs, spring_index + 1);
        }

        // try to read group
        let group_value = groups.get(group_index).map(|g| g.clone()).unwrap();

        let group_possible = (springs.len() - spring_index) >= (group_value as usize)
            && ((spring_index + (group_value as usize)) == springs.len()
                || springs[spring_index + (group_value as usize)].unwrap_or(true))
            && springs[spring_index..spring_index + (group_value as usize)]
                .iter()
                .all(|s| !s.unwrap_or(false));

        if !group_possible {
            return working;
        }

        let group_index = group_index + 1;
        let spring_index = spring_index + (group_value as usize) + 1;

        working + Self::is_valid(groups, group_index, springs, spring_index)
    }

    fn solve(
        i: usize,
        j: usize,
        s: &Vec<char>,
        p: &Vec<char>,
        mut dp_map: HashMap<(usize, usize, String), u128>,
    ) -> (u128, HashMap<(usize, usize, String), u128>) {
        let p_string: String = p.clone().iter().collect();
        let _s_string: String = s.clone().iter().collect();
        // println!("{} \n{}\n{}\n{}\n\n", i, j, s_string, p_string);

        if dp_map.contains_key(&(i, j, p_string.clone())) {
            return (dp_map.get(&(i, j, p_string)).unwrap().clone(), dp_map);
        }

        if i == s.len() && j == p.len() {
            return (1, dp_map);
        }

        if j == p.len() {
            return (0, dp_map);
        }

        if i == s.len() {
            if p[j] == '*' {
                return Self::solve(i, j + 1, s, p, dp_map);
            }
            return (0, dp_map);
        }

        let mut res = 0;
        if (s[i] == '#' || s[i] == '?') && p[j] == '#' {
            (res, dp_map) = Self::solve(i + 1, j + 1, s, p, dp_map);
        } else if (s[i] == '?' || s[i] == '.') && p[j] == '*' {
            let res1;
            let res2;
            (res1, dp_map) = Self::solve(i + 1, j, s, p, dp_map);
            (res2, dp_map) = Self::solve(i, j + 1, s, p, dp_map);
            res = res1 + res2;
        } else if s[i] == '#' && p[j] == '*' {
            (res, dp_map) = Self::solve(i, j + 1, s, p, dp_map);
        } else if (s[i] == '?' || s[i] == '.') && p[j] == '+' {
            let mut x = p.clone();
            x.remove(j);
            x.insert(j, '*');
            (res, dp_map) = Self::solve(i + 1, j, s, &x, dp_map);
        }
        dp_map.insert((i, j, p_string), res);
        return (res, dp_map);
    }
}

#[derive(Debug)]
struct ArrangementParseError {}

impl FromStr for Arrangement {
    type Err = ArrangementParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        let parts: Vec<&str> = s.split(' ').collect();

        let springs: Vec<Option<bool>> = parts[0]
            .chars()
            .map(|c| match c {
                '#' => Some(false),
                '.' => Some(true),
                _ => None,
            })
            .collect();

        let groups: Vec<u128> = parts[1]
            .split(',')
            .map(|n| n.parse::<u128>().unwrap())
            .collect();
        return Ok(Arrangement { springs, groups });
    }
}

#[cfg(test)]
const EXAMPLE_DATA_1: &str = include_str!("../test1.txt");

#[test]
fn example() {
    let part1 = solve_day_12(EXAMPLE_DATA_1);
    assert_eq!(part1, 21);

    let part2 = solve_day12_part2_dp(EXAMPLE_DATA_1);
    assert_eq!(part2, 525152);
}
