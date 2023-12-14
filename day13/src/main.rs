use std::{iter::zip, time::Instant};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now_1 = Instant::now();
    let part1 = solve_day_13_part1(INPUT);

    println!("part1 -> {}", part1);

    let elapsed_1 = now_1.elapsed();
    println!("Part 1 - Elapsed: {:.2?}", elapsed_1);
    let now_2 = Instant::now();
    let part2 = solve_day_13_part2(INPUT);

    println!("part2 -> {}", part2);

    let elapsed_2 = now_2.elapsed();
    println!("Part2 - Elapsed: {:.2?}", elapsed_2);
}

fn solve_day_13_part1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|p| {
            let rows: Vec<String> = p.lines().map(|l| l.to_string()).collect();
            let cols = transpose(&rows);

            let m_r = try_mirror(rows, 0);
            if let Some(m) = m_r {
                return m * 100;
            }
            let m_c = try_mirror(cols, 0);
            if let Some(c) = m_c {
                return c;
            }
            return 0;
        })
        .sum::<usize>() as u32
}

fn solve_day_13_part2(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|p| {
            let rows: Vec<String> = p.lines().map(|l| l.to_string()).collect();
            let cols = transpose(&rows);

            let m_r = try_mirror(rows, 1);
            if let Some(m) = m_r {
                return m * 100;
            }
            let m_c = try_mirror(cols, 1);
            if let Some(c) = m_c {
                return c;
            }
            return 0;
        })
        .sum::<usize>() as u32
}

#[cfg(test)]
const EXAMPLE_DATA_1: &str = include_str!("../test1.txt");

#[test]
fn example() {
    let part1 = solve_day_13_part1(EXAMPLE_DATA_1);
    assert_eq!(part1, 405);

    let part2 = solve_day_13_part2(EXAMPLE_DATA_1);
    assert_eq!(part2, 400);
}

fn str_distance(s1: &String, s2: &String) -> Option<u32> {
    if s1.len() != s2.len() {
        return None;
    }

    Some(
        zip(s1.chars(), s2.chars())
            .map(|(a, b)| (a != b) as u32)
            .sum(),
    )
}

fn try_mirror(lines: Vec<String>, exact_distance: u32) -> Option<usize> {
    (1..lines.len()).find(|m| {
        lines[..m.clone()]
            .iter()
            .enumerate()
            .map(|(i, l)| {
                let pair_index = 2 * m - i - 1;
                if pair_index >= lines.len() {
                    return 0;
                } else {
                    return str_distance(l, &lines[pair_index]).unwrap();
                }
            })
            .sum::<u32>()
            == exact_distance
    })
}

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
