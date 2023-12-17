use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now_1 = Instant::now();
    let part1 = solve_day_16_part1(INPUT);

    println!("part 1 - Result -> {}", part1);

    let elapsed_1 = now_1.elapsed();
    println!("Part 1 - Elapsed: {:.2?}", elapsed_1);

    let now_2 = Instant::now();
    let part2 = solve_day_16_part2(INPUT);

    println!("part2 - Result -> {}", part2);

    let elapsed_2 = now_2.elapsed();
    println!("Part 2 - Elapsed: {:.2?}", elapsed_2);
}

fn solve_day_16_part2(input: &str) -> u64 {
    let grid: HashMap<(i32, i32), Tile> = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.trim()
                .chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), Tile::new(Shape::from(c))))
        })
        .collect();

    let h = grid.keys().map(|p| p.0).max().unwrap() + 1;
    let w = grid.keys().map(|p| p.1).max().unwrap() + 1;

    vec![
        (0..h)
            .map(|i| ((i, 0), Direction::Right))
            .collect::<Vec<((i32, i32), Direction)>>(),
        (0..h)
            .map(|i| ((i, w - 1), Direction::Left))
            .collect::<Vec<((i32, i32), Direction)>>(),
        (0..w)
            .map(|i| ((0, i), Direction::Down))
            .collect::<Vec<((i32, i32), Direction)>>(),
        (0..w)
            .map(|i| ((h - 1, i), Direction::Up))
            .collect::<Vec<((i32, i32), Direction)>>(),
    ]
    .iter()
    .flatten()
    .map(|(pos, dir)| get_energized(grid.clone(), pos, dir))
    .max()
    .unwrap()
}

fn solve_day_16_part1(input: &str) -> u64 {
    let grid: HashMap<(i32, i32), Tile> = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.trim()
                .chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), Tile::new(Shape::from(c))))
        })
        .collect();

    get_energized(grid, &(0, 0), &Direction::Right)
}

fn get_energized(
    mut grid: HashMap<(i32, i32), Tile>,
    start_pos: &(i32, i32),
    dir: &Direction,
) -> u64 {
    let mut beams: VecDeque<Beam> = vec![Beam::new(start_pos.clone(), dir.clone())].into();

    while let Some(beam) = beams.pop_front() {
        // energize tile
        // println!("\n{:?} {:?}", beam.position, beam.direction);

        if let Some(t) = grid.get_mut(&beam.position) {
            t.energize(&beam.direction)
        }

        // move beam
        let next_directions = grid
            .get(&beam.position)
            .map(|t| t.shape.pass_beam(&beam.direction));

        if let Some(next_directions) = next_directions {
            for dir in next_directions {
                let next_position = dir.move_beam(&beam.position);
                // println!("dir {:?}", dir);
                // println!("next_position {:?}", next_position);
                if let Some(t) = grid.get(&next_position) {
                    if !t.is_energized(Some(&dir)) {
                        let new_beam = Beam::new(next_position, dir.clone());
                        // println!("pushed");
                        beams.push_front(new_beam);
                    }
                }
            }
        };
    }

    grid.values().filter(|t| t.is_energized(None)).count() as u64
}

struct Beam {
    position: (i32, i32),
    direction: Direction,
}

impl Beam {
    fn new(position: (i32, i32), direction: Direction) -> Beam {
        Beam {
            position,
            direction,
        }
    }
}

#[derive(Clone)]
struct Tile {
    shape: Shape,
    energizied: HashSet<Direction>,
}

impl Tile {
    fn new(shape: Shape) -> Tile {
        Tile {
            shape,
            energizied: HashSet::<Direction>::new(),
        }
    }

    fn energize(&mut self, dir: &Direction) {
        self.energizied.insert(dir.clone());
    }

    fn is_energized(&self, dir: Option<&Direction>) -> bool {
        match dir {
            Some(dir) => self.energizied.contains(dir),
            None => self.energizied.len() > 0,
        }
    }
}

#[derive(Clone)]
enum Shape {
    Empty,    // .
    Forward,  // /
    Backward, // \
    Straight, // |
    Flat,     // -
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn move_beam(&self, pos: &(i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0, pos.1 + 1),
        }
    }
}

impl Shape {
    fn pass_beam(&self, from: &Direction) -> Vec<Direction> {
        match (self, from) {
            (Shape::Empty, d) => vec![d.clone()],
            (Shape::Forward, Direction::Up) => vec![Direction::Right],
            (Shape::Forward, Direction::Down) => vec![Direction::Left],
            (Shape::Forward, Direction::Left) => vec![Direction::Down],
            (Shape::Forward, Direction::Right) => vec![Direction::Up],
            (Shape::Backward, Direction::Up) => vec![Direction::Left],
            (Shape::Backward, Direction::Down) => vec![Direction::Right],
            (Shape::Backward, Direction::Left) => vec![Direction::Up],
            (Shape::Backward, Direction::Right) => vec![Direction::Down],
            (Shape::Straight, Direction::Left) => vec![Direction::Up, Direction::Down],
            (Shape::Straight, Direction::Right) => vec![Direction::Up, Direction::Down],
            (Shape::Straight, d) => vec![d.clone()],
            (Shape::Flat, Direction::Up) => vec![Direction::Left, Direction::Right],
            (Shape::Flat, Direction::Down) => vec![Direction::Left, Direction::Right],
            (Shape::Flat, d) => vec![d.clone()],
        }
    }
}

impl From<char> for Shape {
    fn from(value: char) -> Self {
        match value {
            '/' => Shape::Forward,
            '\\' => Shape::Backward,
            '-' => Shape::Flat,
            '|' => Shape::Straight,
            _ => Self::Empty,
        }
    }
}

#[cfg(test)]
const EXAMPLE_DATA_1: &str = include_str!("../test1.txt");

#[test]
fn example() {
    let part1 = solve_day_16_part1(EXAMPLE_DATA_1);
    assert_eq!(part1, 46);

    let part2 = solve_day_16_part2(EXAMPLE_DATA_1);
    assert_eq!(part2, 51);
}
