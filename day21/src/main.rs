use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now_1 = Instant::now();
    let part1 = solve_day_20_part1(INPUT, 64);
    let elapsed_1 = now_1.elapsed();
    println!("part 1 - Result -> {}", part1);
    println!("Part 1 - Elapsed: {:.2?}", elapsed_1);

    let now_2 = Instant::now();
    let part2 = solve_day_20_part1(INPUT, 26501365);
    let elapsed_2 = now_2.elapsed();
    println!("part2 - Result -> {}", part2);
    println!("Part 2 - Elapsed: {:.2?}", elapsed_2);

    // // Benchmark
    // println!("\nBenchmark:");
    // let mut part1_v = Vec::<u128>::new();
    // let mut part2_v = Vec::<u128>::new();
    // for _ in 0..1000 {
    //     let now_1 = Instant::now();
    //     let part1 = solve_day_20_part1(INPUT);
    //     let elapsed_1 = now_1.elapsed();
    //     part1_v.push(elapsed_1.as_micros());
    //     let _ = part1;

    //     let now_2 = Instant::now();
    //     let part2 = solve_day_20_part2(INPUT);
    //     let elapsed_2 = now_2.elapsed();
    //     part2_v.push(elapsed_2.as_micros());
    //     let _ = part2;
    // }
    // println!(
    //     "Part 1 - Avarage: {:.2?}",
    //     part1_v.iter().sum::<u128>() / 1000 as u128
    // );
    // println!(
    //     "Part 2 - Avarage: {:.2?}",
    //     part2_v.iter().sum::<u128>() / 1000 as u128
    // );
}

#[cfg(test)]
const EXAMPLE_DATA_1: &str = include_str!("../test1.txt");

#[test]
fn example() {
    let part1 = solve_day_20_part1(EXAMPLE_DATA_1, 6);
    assert_eq!(part1, 16);

    let part1 = solve_day_20_part1(EXAMPLE_DATA_1, 10);
    assert_eq!(part1, 50);

    let part1 = solve_day_20_part1(EXAMPLE_DATA_1, 100);
    assert_eq!(part1, 6536);

    let part1 = solve_day_20_part1(EXAMPLE_DATA_1, 5000);
    assert_eq!(part1, 16733044);
}

#[derive(PartialEq, Eq)]
enum Ground {
    Garden,
    Rock,
}

fn move_from_pos(pos: &(i64, i64)) -> Vec<(i64, i64)> {
    return vec![
        (pos.0 + 1, pos.1),
        (pos.0 - 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0, pos.1 - 1),
    ];
}

fn solve_day_20_part1(input: &str, max_steps: u32) -> u128 {
    let mut active: HashSet<(i64, i64)> = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.trim()
                .chars()
                .enumerate()
                .flat_map(move |(j, c)| match c {
                    'S' => Some((i as i64, j as i64)),
                    _ => None,
                })
        })
        .collect::<HashSet<(i64, i64)>>();
    let bool_start = max_steps % 2 == 0;
    let mut full_map: HashMap<(i64, i64), (Ground, u128)> = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.trim().chars().enumerate().map(move |(j, c)| {
                (
                    (i as i64, j as i64),
                    (
                        match c {
                            '#' => Ground::Rock,
                            _ => Ground::Garden,
                        },
                        0,
                    ),
                )
            })
        })
        .collect();

    let down_margin = full_map.keys().map(|(i, _)| i).max().unwrap() + 1;
    let right_margin = full_map.keys().map(|(_, j)| j).max().unwrap() + 1;

    let mut visited = HashSet::<(i64, i64)>::new();

    for step in 1..max_steps + 1 {
        if step % 10000 == 0 {
            println!("{step}");
        }
        let new_active: HashSet<(i64, i64)> = active
            .iter()
            .flat_map(|a| move_from_pos(a))
            .filter(|a| {
                let real_pos = convert(a, down_margin, right_margin);
                if full_map.get(&real_pos).is_none() {
                    println!("{a:?}, {real_pos:?}");
                }
                !visited.contains(a)
                    && match full_map.get(&real_pos).unwrap().0 {
                        Ground::Garden => true,
                        Ground::Rock => false,
                    }
            })
            .collect();

        // println!("new active: {active:?}");

        // process active
        // -> remove if already visited
        // -> count + 1 in real_pos if step == starting bool
        for a in new_active.iter() {
            if !visited.contains(&a) && (step % 2 == 0) == bool_start {
                let real_pos = &convert(&a, down_margin, right_margin);
                let real_pos_cnt = full_map.get(real_pos).unwrap().1 + 1;
                // println!("{step} {real_pos:?} -> {real_pos_cnt}");
                full_map.insert(real_pos.clone(), (Ground::Garden, real_pos_cnt));
            }
        }
        visited.extend(new_active.clone());

        active = new_active;
    }

    full_map.values().map(|(_, count)| count).sum()
}

fn convert(pos: &(i64, i64), d_margin: i64, r_margin: i64) -> (i64, i64) {
    let d = pos.0 % d_margin;
    let r = pos.1 % r_margin;
    (d.abs(), r.abs())
}
