use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
    time::Instant,
    vec,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now_1 = Instant::now();
    let part1 = solve_day_23_part1(INPUT, (0, 1), (140, 139));
    let elapsed_1 = now_1.elapsed();
    println!("part 1 - Result -> {}", part1);
    println!("Part 1 - Elapsed: {:.2?}", elapsed_1);

    // let now_2 = Instant::now();
    // let part2 = solve_day_23_part1(INPUT, 26501365);
    // let elapsed_2 = now_2.elapsed();
    // println!("part2 - Result -> {}", part2);
    // println!("Part 2 - Elapsed: {:.2?}", elapsed_2);

    // // Benchmark
    // println!("\nBenchmark:");
    // let mut part1_v = Vec::<u128>::new();
    // let mut part2_v = Vec::<u128>::new();
    // for _ in 0..1000 {
    //     let now_1 = Instant::now();
    //     let part1 = solve_day_23_part1(INPUT);
    //     let elapsed_1 = now_1.elapsed();
    //     part1_v.push(elapsed_1.as_micros());
    //     let _ = part1;

    //     let now_2 = Instant::now();
    //     let part2 = solve_day_23_part2(INPUT);
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
    let part1 = solve_day_23_part1(EXAMPLE_DATA_1, (0, 1), (22, 21));
    assert_eq!(part1, 94);
}

#[derive(PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_as_vec() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
}

impl Direction {
    fn walk(&self, pos: (i64, i64)) -> (i64, i64) {
        match self {
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0, pos.1 + 1),
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

struct Path {
    past: HashSet<(i64, i64)>,
    current: (i64, i64),
}

impl Path {
    fn walk(&mut self, map: &Map) -> Option<Self> {
        match map.tiles.get(&self.current)? {
            Tile::Path => {
                // look for more than one dir
                let mut next_pos: Vec<(i64, i64)> = Direction::get_as_vec()
                    .iter()
                    .map(|d| {
                        let pos = d.walk(self.current);
                        let possible_pos = match map.tiles.get(&pos) {
                            Some(t) => match t {
                                Tile::Path => true,
                                Tile::Forest => false,
                                Tile::Slope(slope_dir) => slope_dir == d,
                            },
                            None => false,
                        };
                        (pos, possible_pos & !self.past.contains(&pos))
                    })
                    .filter(|(_, pp)| pp.clone())
                    .map(|(p, _)| p)
                    .collect();

                self.past.insert(self.current);
                let next_of_current = next_pos.pop()?;
                self.current = next_of_current;

                let other_path = next_pos.pop()?;
                Some(Path {
                    past: self.past.clone(),
                    current: other_path,
                })
            }
            Tile::Forest => None,
            Tile::Slope(d) => {
                self.past.insert(self.current);
                let next_pos = d.walk(self.current);
                if !self.past.contains(&next_pos)
                    && map.tiles.get(&next_pos).unwrap_or(&Tile::Forest) != &Tile::Forest
                {
                    self.current = next_pos
                }
                None
            }
        }
    }
}

struct Map {
    tiles: HashMap<(i64, i64), Tile>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: HashMap<(i64, i64), Tile> = s
            .lines()
            .enumerate()
            .flat_map(|(i, l)| {
                l.trim().chars().enumerate().map(move |(j, c)| {
                    ((i as i64, j as i64), {
                        match c {
                            '.' => Tile::Path,
                            '>' => Tile::Slope(Direction::Right),
                            'v' => Tile::Slope(Direction::Down),
                            '<' => Tile::Slope(Direction::Left),
                            '^' => Tile::Slope(Direction::Up),
                            _ => Tile::Forest,
                        }
                    })
                })
            })
            .collect();

        Ok(Map { tiles })
    }
}

fn solve_day_23_part1(input: &str, start_pos: (i64, i64), end_pos: (i64, i64)) -> u64 {
    let map: Map = Map::from_str(input).unwrap();
    let mut paths = VecDeque::<Path>::new();

    let mut results = VecDeque::<Path>::new();

    paths.push_front(Path {
        past: HashSet::<(i64, i64)>::new(),
        current: start_pos,
    });

    while let Some(mut p) = paths.pop_back() {
        let start_pos = p.current;
        let new_p = p.walk(&map);
        // println!("\n{start_pos:?} -> {:?}", p.current);
        if p.current != start_pos {
            if p.current == end_pos {
                results.push_front(p)
            } else {
                paths.push_front(p);
            }
        }
        // else {
        //     println!("STOP!!!!")
        // }
        if let Some(new_p) = new_p {
            paths.push_front(new_p);
        }
    }

    results.iter().map(|r| r.past.len()).max().unwrap() as u64
}
