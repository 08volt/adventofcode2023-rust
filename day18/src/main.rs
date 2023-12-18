use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

use sorted_vec::SortedSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    // let now_1 = Instant::now();
    // let part1 = solve_day_17_part1(INPUT);

    // println!("part 1 - Result -> {}", part1);

    // let elapsed_1 = now_1.elapsed();
    // println!("Part 1 - Elapsed: {:.2?}", elapsed_1);

    let now_2 = Instant::now();
    let part2 = solve_day_17_part2(INPUT);

    println!("part2 - Result -> {}", part2);

    let elapsed_2 = now_2.elapsed();
    println!("Part 2 - Elapsed: {:.2?}", elapsed_2);
}

#[derive(Clone, Debug)]
struct Row {
    limits: SortedSet<i64>,
}

impl Row {
    fn new() -> Row {
        Row {
            limits: SortedSet::<i64>::new(),
        }
    }

    fn count_area(&self) -> u64 {
        let mut area = false;
        let mut last = 0;
        let mut count: u64 = 0;
        let mut limits = self.limits.clone();

        while let Some(c) = limits.pop() {
            if !area {
                last = c;
                area = true;
            } else {
                count += (last - c + 1) as u64;
                area = false;
            }
        }

        if area {
            println!("area is true");
        }

        count
    }
}

fn solve_day_17_part2(input: &str) -> u64 {
    let moves: Vec<DigMove> = input
        .lines()
        .map(|l| l.trim().try_into().unwrap())
        .collect();

    // println!("digged = {:?}", digged);
    let digged: HashSet<(i32, i32)> = dig_area(moves);

    let d: usize = (digged.iter().map(|k| k.0).max().unwrap()
        - digged.iter().map(|k| k.0).min().unwrap()) as usize
        + 1;

    let u: i32 = digged.iter().map(|k| k.0).min().unwrap();

    let vertical_limits: Vec<&(i32, i32)> = digged
        .iter()
        .filter(|(r, c)| {
            let rc = (r.clone(), c.clone());
            let rcm1 = (r.clone(), c - 1);
            let rcp1 = (r.clone(), c + 1);
            (digged.contains(&rc) && !digged.contains(&rcp1))
                || (!digged.contains(&rc) && digged.contains(&rcm1))
        })
        .collect();

    let mut r_limits: Vec<Row> = vec![Row::new(); d];
    println!("d {} ", d);
    vertical_limits.iter().for_each(|(r, c)| {
        // println!("limit {:?} ", (r + u));
        let r = r_limits.get_mut((r + u) as usize).unwrap();
        r.limits.push(c.clone() as i64);
    });

    r_limits.iter().map(|r| r.count_area()).sum()
}

fn solve_day_17_part1(input: &str) -> u64 {
    let moves: Vec<DigMove> = input
        .lines()
        .map(|l| l.trim().try_into().unwrap())
        .collect();

    // println!("digged = {:?}", digged);
    let mut digged: HashSet<(i32, i32)> = dig_area(moves);

    println!("Digged {} meters from moves", digged.len());

    let d = digged.iter().map(|k| k.0).max().unwrap() + 1;
    let r = digged.iter().map(|k| k.1).max().unwrap() + 1;

    let u = digged.iter().map(|k| k.0).min().unwrap() - 1;
    let l = digged.iter().map(|k| k.1).min().unwrap() - 1;

    let limits: (i32, i32, i32, i32) = (d, r, u, l);
    println!("limits {:?}", limits);
    let mut not_digged = HashSet::<(i32, i32)>::new();

    (u + 2..d - 1).for_each(|y| {
        (l + 2..r - 1).for_each(|x| {
            let (explored, dig) = explore_area(&digged, &not_digged, &(y, x), &limits);
            match dig {
                true => explored.iter().for_each(|d| {
                    digged.insert(d.clone());
                }),
                false => explored.iter().for_each(|d| {
                    not_digged.insert(d.clone());
                }),
            }
        });
    });

    println!("Total Digged {} meters", digged.len());
    println!("Total Not Digged {} meters", not_digged.len());
    digged.len() as u64
}

fn dig_area(moves: Vec<DigMove>) -> HashSet<(i32, i32)> {
    let starting_pos: (i32, i32) = (0, 0);

    let mut digged: HashSet<(i32, i32)> = HashSet::new();
    let final_pos = moves.iter().fold(starting_pos, |pos, b| {
        let get_dig = b.direction.get_steps(&pos, b.steps);
        get_dig.iter().for_each(|d| {
            digged.insert(d.clone());
        });
        get_dig.iter().last().unwrap_or(&starting_pos).clone()
    });

    if final_pos != starting_pos {
        println!("!ALERT!: Dig Starting position is different from Dig final position");
    }

    digged
}

fn explore_area(
    digged: &HashSet<(i32, i32)>,
    not_digged: &HashSet<(i32, i32)>,
    starting_pos: &(i32, i32),
    limits: &(i32, i32, i32, i32),
) -> (HashSet<(i32, i32)>, bool) {
    let mut explored = HashSet::<(i32, i32)>::new();
    let mut explore_list: VecDeque<(i32, i32)> = VecDeque::new();
    explore_list.push_back(starting_pos.clone());

    while let Some(pos) = explore_list.pop_front() {
        if not_digged.contains(&pos) || is_limit(&pos, &limits) {
            explored.insert(pos);
            return (explored, false);
        }
        if !digged.contains(&pos) && !explored.contains(&pos) {
            explore_list.push_back(Direction::Up.move_dir(&pos));
            explore_list.push_back(Direction::Down.move_dir(&pos));
            explore_list.push_back(Direction::Left.move_dir(&pos));
            explore_list.push_back(Direction::Right.move_dir(&pos));
        }
        explored.insert(pos);
    }
    (explored, true)
}

fn is_limit(pos: &(i32, i32), limits: &(i32, i32, i32, i32)) -> bool {
    pos.0 >= limits.0 || pos.0 <= limits.2 || pos.1 >= limits.1 || pos.1 <= limits.3
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn move_dir(&self, pos: &(i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Right => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0, pos.1 - 1),
        }
    }
    fn get_steps(&self, pos: &(i32, i32), count: u64) -> Vec<(i32, i32)> {
        let mut pos = pos.clone();
        let mut res = Vec::<(i32, i32)>::new();
        res.push(pos);
        for _ in 0..count {
            pos = self.move_dir(&pos);
            res.push(pos);
        }

        res
    }
}

impl TryFrom<&str> for Direction {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            "D" => Ok(Direction::Down),
            "U" => Ok(Direction::Up),
            _ => Err(ParseError {}),
        }
    }
}

#[derive(Debug, Clone)]
struct DigMove {
    direction: Direction,
    steps: u64,
    // color: String,
}

#[derive(Debug)]
struct ParseError {}

impl TryFrom<&str> for DigMove {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        print!("{} ", value);
        if let Some((dir, steps)) = value.split_once(' ') {
            if let Some((steps, color)) = steps.split_once(' ') {
                // let _: u64 = steps.parse().map_err(|_| ParseError {})?;
                let color = color;

                let mut number_hex: Vec<char> = color.chars().filter(|c| c.is_digit(16)).collect();

                let direction = number_hex
                    .pop()
                    .map(|c| match c {
                        '0' => Direction::Right,
                        '1' => Direction::Down,
                        '2' => Direction::Left,
                        _ => Direction::Up,
                    })
                    .ok_or(ParseError {})?;

                let mut steps: u64 = 0;
                let mut i: u32 = 0;

                while let Some(c) = number_hex.pop() {
                    steps += c.to_digit(16).unwrap() as u64 * ((16 as u64).pow(i));
                    i += 1;
                }

                print!("{}\n", steps);

                // let direction = dir.try_into()?;
                return Ok(DigMove { direction, steps });
            }
        }
        Err(ParseError {})
    }
}

#[cfg(test)]
const EXAMPLE_DATA_1: &str = include_str!("../test1.txt");

#[test]
fn example() {
    // let part1 = solve_day_17_part1(EXAMPLE_DATA_1);
    // assert_eq!(part1, 62);

    let part2 = solve_day_17_part2(EXAMPLE_DATA_1);
    assert_eq!(part2, 952408144115);
}
