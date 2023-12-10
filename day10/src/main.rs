use std::collections::{HashMap, HashSet};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part1 = solve_day_10(INPUT);

    println!("{}", part1);

    let part2 = solve_day_10_part2(INPUT);

    println!("{}", part2);
}

fn find_start(input: &str) -> Option<(i32, i32)> {
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.trim().chars().enumerate() {
            if c == 'S' {
                return Some((x as i32, y as i32));
            }
        }
    }
    None
}

fn solve_day_10(input: &str) -> i32 {
    if let Some(start) = find_start(&input) {
        let map: HashMap<(i32, i32), Pipe> = input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                let map: HashMap<(i32, i32), Pipe> = l
                    .trim()
                    .chars()
                    .enumerate()
                    .map(|(x, c)| ((x as i32, y as i32), Pipe::new(c)))
                    .collect();
                map
            })
            .collect();

        // println!("{:?}", map);

        let mut current_steps: Vec<Step> = Direction::iter()
            .filter(|d| {
                map.get(&d.go(&start))
                    .map(|p| p.is_connected_from(d))
                    .unwrap_or(false)
            })
            .map(|d| {
                let mut ph = Vec::<(i32, i32)>::new();
                ph.push(d.go(&start));
                Step {
                    number: 1,
                    last_direction: d,
                    position_history: ph,
                }
            })
            .collect();

        // println!("\n{:?}\n", current_steps);

        let mut i = 0;

        while !(&current_steps)
            .windows(2)
            .map(|s| {
                HashSet::<&(i32, i32)>::from_iter(&s[0].position_history)
                    .intersection(&HashSet::<&(i32, i32)>::from_iter(&s[1].position_history))
                    .next()
                    .is_some()
            })
            .fold(false, |a, b| a || b)
        {
            // println!("{}", i);
            i += 1;
            for s in current_steps.iter_mut() {
                s.walk(&map, &start);
            }
        }

        return current_steps.iter().map(|s| s.number).max().unwrap_or(-1);
    }

    0
}

fn solve_day_10_part2(input: &str) -> i32 {
    if let Some(start) = find_start(&input) {
        let mut map: HashMap<(i32, i32), Pipe> = input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                let map: HashMap<(i32, i32), Pipe> = l
                    .trim()
                    .chars()
                    .enumerate()
                    .map(|(x, c)| ((x as i32, y as i32), Pipe::new(c)))
                    .collect();
                map
            })
            .collect();

        let mut current_steps: Vec<Step> = Direction::iter()
            .filter(|d| {
                map.get(&d.go(&start))
                    .map(|p| p.is_connected_from(d))
                    .unwrap_or(false)
            })
            .map(|d| {
                let mut ph = Vec::<(i32, i32)>::new();
                ph.push(d.go(&start));
                Step {
                    number: 1,
                    last_direction: d,
                    position_history: ph,
                }
            })
            .collect();

        while !(&current_steps)
            .windows(2)
            .map(|s| {
                HashSet::<&(i32, i32)>::from_iter(&s[0].position_history)
                    .intersection(&HashSet::<&(i32, i32)>::from_iter(&s[1].position_history))
                    .next()
                    .is_some()
            })
            .fold(false, |a, b| a || b)
        {
            for s in current_steps.iter_mut() {
                s.walk(&map, &start);
            }
        }

        let mut closed_loop: HashSet<(i32, i32)> = current_steps
            .iter()
            .fold(HashSet::<&(i32, i32)>::new(), |a, b| {
                a.union(&HashSet::from_iter(b.position_history.iter()))
                    .map(|&a| a)
                    .collect()
            })
            .iter()
            .map(|&(a, b)| (a.clone(), b.clone()))
            .collect();

        closed_loop.insert(start);

        let mut count = 0;

        let start_pipe: HashSet<Direction> = Direction::iter()
            .filter(|d| {
                map.get(&d.go(&start))
                    .map(|p| p.is_connected_from(d))
                    .unwrap_or(false)
            })
            .collect();

        println!("start Pipe {:?} ", start_pipe);

        for (y, l) in input.lines().enumerate() {
            let mut flags = HashSet::<Direction>::new();

            for (x, c) in l.chars().enumerate() {
                let p = match c {
                    'S' => Pipe::fromIter(&start_pipe),
                    _ => Pipe::new(c),
                };

                if closed_loop.contains(&(x as i32, y as i32)) {
                    println!("{:?} | {}", (x, y), c);
                    for d in p.connections {
                        if flags.contains(&d) {
                            flags.remove(&d);
                        } else {
                            flags.insert(d);
                        }
                    }
                } else if flags.contains(&Direction::North) && flags.contains(&Direction::South) {
                    count += 1;
                    println!("{:?}", (x, y))
                }
            }
        }

        return count;
    }

    0
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Step {
    number: i32,
    last_direction: Direction,
    position_history: Vec<(i32, i32)>,
}

impl Step {
    fn walk(&mut self, map: &HashMap<(i32, i32), Pipe>, start: &(i32, i32)) {
        self.last_direction = map
            .get(&self.position_history.last().unwrap_or(start))
            .map(|p| {
                p.connections
                    .iter()
                    .filter(|&d| d != &self.last_direction.opposite())
                    .next()
                    .map(|d| d.clone())
                    .unwrap_or_default()
            })
            .unwrap_or_default();
        self.position_history.push(
            self.last_direction
                .go(&self.position_history.last().unwrap_or(start)),
        );
        self.number += 1;
        // println!("{:?}", self);
    }
}

#[derive(Debug)]
struct Pipe {
    connections: HashSet<Direction>,
}

impl Pipe {
    fn is_connected_from(&self, coming: &Direction) -> bool {
        self.connections.contains(&coming.opposite())
    }

    fn new(kind: char) -> Pipe {
        let mut connections = HashSet::new();
        match kind {
            '|' => {
                connections.insert(Direction::North);
                connections.insert(Direction::South);
            }
            '-' => {
                connections.insert(Direction::East);
                connections.insert(Direction::West);
            }
            'L' => {
                connections.insert(Direction::North);
                connections.insert(Direction::East);
            }
            'J' => {
                connections.insert(Direction::North);
                connections.insert(Direction::West);
            }
            '7' => {
                connections.insert(Direction::West);
                connections.insert(Direction::South);
            }
            'F' => {
                connections.insert(Direction::East);
                connections.insert(Direction::South);
            }
            _ => {}
        }
        Pipe { connections }
    }

    fn fromIter(a: &HashSet<Direction>) -> Self {
        Pipe {
            connections: a.clone(),
        }
    }
}

#[derive(PartialEq, Clone, Eq, Hash, EnumIter, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
    Stop,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Stop
    }
}

impl Direction {
    fn value(&self) -> (i32, i32) {
        match &self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
            Direction::Stop => (0, 0),
        }
    }

    fn go(&self, position: &(i32, i32)) -> (i32, i32) {
        let dir = self.value();
        (position.0 + dir.0, position.1 + dir.1)
    }

    fn opposite(&self) -> Direction {
        match &self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::Stop => Direction::Stop,
        }
    }
}

impl Pipe {}

#[cfg(test)]
const EXAMPLE_DATA_1: &str = include_str!("../test1.txt");
#[cfg(test)]
const EXAMPLE_DATA_2: &str = include_str!("../test2.txt");
#[cfg(test)]
const EXAMPLE_DATA_3: &str = include_str!("../test3.txt");
#[cfg(test)]
const EXAMPLE_DATA_4: &str = include_str!("../test4.txt");
#[cfg(test)]
const EXAMPLE_DATA_5: &str = include_str!("../test5.txt");

#[test]
fn example() {
    // let part1 = solve_day_10(EXAMPLE_DATA_1);
    // assert_eq!(part1, 4);
    // let part1 = solve_day_10(EXAMPLE_DATA_2);
    // assert_eq!(part1, 8);

    // let part2 = solve_day_10_part2(EXAMPLE_DATA_3);
    // assert_eq!(part2, 4);

    // let part2 = solve_day_10_part2(EXAMPLE_DATA_4);
    // assert_eq!(part2, 10);

    let part2 = solve_day_10_part2(EXAMPLE_DATA_5);
    assert_eq!(part2, 8);
}
