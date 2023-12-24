use std::{str::FromStr, time::Instant};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now_1 = Instant::now();
    let part1 = solve_day_24_part1(INPUT, (200000000000000_f64, 400000000000000_f64));
    let elapsed_1 = now_1.elapsed();
    println!("part 1 - Result -> {}", part1);
    println!("Part 1 - Elapsed: {:.2?}", elapsed_1);

    // let now_2 = Instant::now();
    // let part2 = solve_day_24_part2(INPUT, (0, 1), (140, 139));
    // let elapsed_2 = now_2.elapsed();
    // println!("part2 - Result -> {}", part2);
    // println!("Part 2 - Elapsed: {:.2?}", elapsed_2);

    // // Benchmark
    // println!("\nBenchmark:");
    // let mut part1_v = Vec::<u128>::new();
    // let mut part2_v = Vec::<u128>::new();
    // for _ in 0..1000 {
    //     let now_1 = Instant::now();
    //     let part1 = solve_day_24_part1(INPUT);
    //     let elapsed_1 = now_1.elapsed();
    //     part1_v.push(elapsed_1.as_micros());
    //     let _ = part1;

    //     let now_2 = Instant::now();
    //     let part2 = solve_day_24_part2(INPUT);
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
    let part1 = solve_day_24_part1(EXAMPLE_DATA_1, (7_f64, 27_f64));
    assert_eq!(part1, 2);

    // let part2 = solve_day_24_part2(EXAMPLE_DATA_1, (0, 1), (22, 21));
    // assert_eq!(part2, 154);
}
struct Stone {
    pos: (f64, f64, f64),
    v: (f64, f64, f64),
}

impl Stone {
    fn cross_at(&self, other: &Self) -> Option<(f64, f64)> {
        println!("\n{}", Into::<String>::into(self));
        println!("{}", Into::<String>::into(other));

        if (self.v.0 * other.v.1 / (self.v.1 * other.v.0) - 1_f64) == 0_f64 {
            return None;
        }

        let y = ((self.pos.1 * self.v.0 / self.v.1 + self.pos.0 - other.pos.0) * other.v.1
            / other.v.0
            + other.pos.1)
            / (self.v.0 * other.v.1 / (self.v.1 * other.v.0) - 1_f64);

        let x = y * self.v.0 / self.v.1 - self.pos.1 * self.v.0 / self.v.1 + self.pos.0;

        Some((x, y))
    }
}

impl Into<String> for &Stone {
    fn into(self) -> String {
        format!("{:?} @ {:?}", self.pos, self.v)
    }
}

impl FromStr for Stone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos_str, v_str) = s.split_once("@").unwrap();

        let pos_v: Vec<f64> = pos_str
            .split(",")
            .map(|n| n.trim().parse::<f64>().unwrap())
            .collect();
        let v_v: Vec<f64> = v_str
            .split(",")
            .map(|n| n.trim().parse::<f64>().unwrap())
            .collect::<Vec<f64>>();

        Ok(Stone {
            pos: (pos_v[0], pos_v[1], pos_v[2]),
            v: (v_v[0], v_v[1], v_v[2]),
        })
    }
}

fn solve_day_24_part1(input: &str, test_area: (f64, f64)) -> u64 {
    let stones: Vec<Stone> = input.lines().map(|l| Stone::from_str(l).unwrap()).collect();

    let mut count: u64 = 0;

    for (i, s) in stones.iter().enumerate() {
        if i == stones.len() - 1 {
            break;
        }
        for s2 in stones[i + 1..].into_iter() {
            let cross = s.cross_at(s2);
            if let Some(cross) = cross {
                if cross.0 > test_area.0
                    && cross.0 < test_area.1
                    && cross.1 > test_area.0
                    && cross.1 < test_area.1
                {
                    println!("FOUND! {:?}", cross);
                    count += 1;
                }
            }
        }
    }

    count
}

// fn solve_day_24_part2(input: &str, start_pos: (f64, f64), end_pos: (f64, f64)) -> u64 {
//     0
// }
