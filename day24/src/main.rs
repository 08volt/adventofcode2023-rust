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

    let part2 = solve_day_24_part2(EXAMPLE_DATA_1);
    assert_eq!(part2, 47);
}

#[derive(Debug)]
struct Stone {
    pos: (f64, f64, f64),
    v: (f64, f64, f64),
}

impl Stone {
    fn is_parallel(&self, other: &Self) -> bool {
        (self.v.0 * other.v.1 / (self.v.1 * other.v.0) - 1_f64) == 0_f64
    }

    fn cross_at(&self, other: &Self) -> Option<(f64, f64)> {
        // println!("\n{}", Into::<String>::into(self));
        // println!("{}", Into::<String>::into(other));

        if self.is_parallel(other) {
            return None; // parallele
        }

        let a = self.pos.0;
        let b = self.v.0;
        let c = self.pos.1;
        let d = self.v.1;

        let e = other.pos.0;
        let f = other.v.0;
        let g = other.pos.1;
        let h = other.v.1;

        let x = (b * f * g + a * d * f - h * e * b - c * b * f) / (d * f - h * b);
        let y = c + (d / b) * (x - a);

        if self.is_past((x, y)) || other.is_past((x, y)) {
            None
        } else {
            Some((x, y))
        }
    }

    fn is_past(&self, pos: (f64, f64)) -> bool {
        (pos.0 - self.pos.0) * self.v.0 < 0_f64 || (pos.1 - self.pos.1) * self.v.1 < 0_f64
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
                    // println!("FOUND! {:?}", cross);
                    count += 1;
                }
            }
        }
    }

    count
}

fn solve_day_24_part2(input: &str) -> u64 {
    let stones: Vec<Stone> = input.lines().map(|l| Stone::from_str(l).unwrap()).collect();

    let mut parallels = Vec::<(&Stone, &Stone)>::new();

    while parallels.len() < 2 {
        for (i, s) in stones.iter().enumerate() {
            println!("\n{}", Into::<String>::into(s));
            for s2 in stones.iter().skip(i + 1) {
                if s.is_parallel(s2) {
                    println!("FOUND {:?}", (s, s2));
                    parallels.push((s, s2));
                    break;
                }
            }
        }
    }

    let x1 = parallels[0].0.pos.0;
    let y1 = parallels[0].0.pos.1;
    let z1 = parallels[0].0.pos.2;
    let x2 = parallels[0].1.pos.0;
    let y2 = parallels[0].1.pos.1;
    let z2 = parallels[0].1.pos.1;
    let x3 = parallels[0].1.pos.0 + parallels[0].1.v.0;
    let y3 = parallels[0].1.pos.1 + parallels[0].1.v.1;
    let z3 = parallels[0].1.pos.2 + parallels[0].1.v.2;

    let a = (y2 - y1) * (z3 - z1) - (z2 - z1) * (y3 - y1);
    let b = (z2 - z1) * (x3 - x1) - (x2 - x1) * (z3 - z1);
    let c = (x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1);

    let d = a * x1 + b * x2 + c * z1;

    0
}
