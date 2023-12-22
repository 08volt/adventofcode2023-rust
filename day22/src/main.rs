use std::{collections::HashSet, str::FromStr, time::Instant};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now_1 = Instant::now();
    let part1 = solve_day_20_part1(INPUT);
    let elapsed_1 = now_1.elapsed();
    println!("part 1 - Result -> {}", part1);
    println!("Part 1 - Elapsed: {:.2?}", elapsed_1);

    // let now_2 = Instant::now();
    // let part2 = solve_day_20_part1(INPUT, 26501365);
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
    let part1 = solve_day_20_part1(EXAMPLE_DATA_1);
    assert_eq!(part1, 5);
}

#[derive(Clone)]
struct Brick {
    block_positions: HashSet<(u32, u32, u32)>,
}

impl Brick {
    fn can_fall(&self, occupied_positions: &HashSet<(u32, u32, u32)>) -> bool {
        self.block_positions.iter().all(|pos| {
            let new_pos = (pos.0, pos.1, pos.2 - 1);

            pos.2 > 1
                && (self.block_positions.contains(&new_pos)
                    || !occupied_positions.contains(&new_pos))
        })
    }

    fn go_down(&mut self, occupied_positions: &HashSet<(u32, u32, u32)>) -> Vec<(u32, u32, u32)> {
        if self.can_fall(occupied_positions) {
            // println!("{:?} can fall", self.block_positions);

            self.block_positions = self
                .block_positions
                .iter()
                .map(|pos| (pos.0, pos.1, pos.2 - 1))
                .collect();

            // println!("{:?} new position\n", self.block_positions);
        }

        self.block_positions
            .iter()
            .map(|p: &(u32, u32, u32)| p.clone())
            .collect()
    }

    fn safe(&self, bricks: &Vec<Brick>) -> bool {
        let new_bricks: Vec<Brick> = bricks
            .into_iter()
            .filter(|&b| {
                !b.block_positions
                    .iter()
                    .any(|p| self.block_positions.contains(p))
            })
            .map(|b| b.clone())
            .collect();
        let positions = Self::compute_positions(&new_bricks);
        let safe = new_bricks.into_iter().all(|b| !b.can_fall(&positions));
        // println!("block {:?} is safe: {}", self.block_positions, safe);
        safe
    }

    fn compute_positions(bricks: &Vec<Brick>) -> HashSet<(u32, u32, u32)> {
        let start_position: HashSet<(u32, u32, u32)> = bricks
            .into_iter()
            .flat_map(|b| b.block_positions.iter().map(|p| p.clone()))
            .collect();
        start_position
    }
}

impl FromStr for Brick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut block_positions = HashSet::<(u32, u32, u32)>::new();

        let (start, end) = s.trim().split_once("~").unwrap();

        let vec_start = start
            .split(",")
            .map(|c| c.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let vec_end = end
            .split(",")
            .map(|c| c.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        (vec_start[0]..vec_end[0] + 1).for_each(|v_0| {
            (vec_start[1]..vec_end[1] + 1).for_each(|v_1| {
                (vec_start[2]..vec_end[2] + 1).for_each(|v_2| {
                    block_positions.insert((v_0, v_1, v_2));
                });
            });
        });

        Ok(Brick { block_positions })
    }
}

fn solve_day_20_part1(input: &str) -> u32 {
    let mut bricks: Vec<Brick> = input
        .lines()
        .map(|l| Brick::from_str(&l).unwrap())
        .collect();

    let mut can_fall = true;

    while can_fall {
        let start_position = Brick::compute_positions(&bricks);
        // println!("start_positions= {start_position:?}\n");
        let mut new_positions = HashSet::<(u32, u32, u32)>::new();

        bricks.iter_mut().for_each(|b| {
            let new_blk_positions = b.go_down(&start_position);
            new_positions.extend(new_blk_positions);
        });

        can_fall = start_position.difference(&new_positions).count() > 0;
    }

    bricks.iter().filter(|b| b.safe(&bricks)).count() as u32
}
