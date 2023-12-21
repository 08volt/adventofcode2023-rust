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

    // let now_2 = Instant::now();
    // let part2 = solve_day_20_part2(INPUT);
    // let elapsed_2 = now_2.elapsed();
    // println!("part2 - Result -> {}", part2);
    // println!("Part 2 - Elapsed: {:.2?}", elapsed_2);

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

    //     let part2 = solve_day_20_part2(EXAMPLE_DATA_1);
    //     assert_eq!(part2, 167409079868000);
}

#[derive(PartialEq, Eq)]
enum Ground {
    Garden,
    Rock,
}

fn move_from_pos(pos: &(i32, i32)) -> HashSet<(i32, i32)> {
    return vec![
        (pos.0 + 1, pos.1),
        (pos.0 - 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0, pos.1 - 1),
    ]
    .iter()
    .map(|x| x.clone())
    .collect();
}

fn solve_day_20_part1(input: &str, max_steps: u32) -> u128 {
    let mut full_map: HashMap<(i32, i32), (Ground, bool)> = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.trim().chars().enumerate().map(move |(j, c)| {
                (
                    (i as i32, j as i32),
                    (
                        match c {
                            '#' => Ground::Rock,
                            _ => Ground::Garden,
                        },
                        match c {
                            'S' => true,
                            _ => false,
                        },
                    ),
                )
            })
        })
        .collect();

    (0..max_steps).for_each(|_| {
        let mut new_map = HashMap::<(i32, i32), (Ground, bool)>::new();
        for pos in full_map.keys() {
            if full_map
                .get(pos)
                .map(|(g, _)| match g {
                    Ground::Rock => false,
                    Ground::Garden => true,
                })
                .unwrap_or(false)
                && move_from_pos(pos)
                    .iter()
                    .any(|p| full_map.get(p).map(|(_, b)| b.clone()).unwrap_or(false))
            {
                new_map.insert(pos.clone(), (Ground::Garden, true));
            } else {
                new_map.insert(
                    pos.clone(),
                    (
                        full_map
                            .get(pos)
                            .map(|(g, _)| match g {
                                Ground::Rock => Ground::Rock,
                                Ground::Garden => Ground::Garden,
                            })
                            .unwrap_or(Ground::Rock),
                        false,
                    ),
                );
            }
        }
        full_map = new_map;
    });

    full_map.values().filter(|(_, b)| b.clone()).count() as u128
}

// fn solve_day_20_part2(input: &str) -> u128 {
//     0
// }
