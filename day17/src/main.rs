use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now_1 = Instant::now();
    let part1 = solve_day_17_part1(INPUT);

    println!("part 1 - Result -> {}", part1);

    let elapsed_1 = now_1.elapsed();
    println!("Part 1 - Elapsed: {:.2?}", elapsed_1);

    let now_2 = Instant::now();
    let part2 = solve_day_17_part2(INPUT);

    println!("part2 - Result -> {}", part2);

    let elapsed_2 = now_2.elapsed();
    println!("Part 2 - Elapsed: {:.2?}", elapsed_2);
}

fn solve_day_17_part2(input: &str) -> u32 {
    let grid: HashMap<(usize, usize), u32> = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(move |(j, c)| ((i, j), c.to_digit(10).unwrap()))
        })
        .collect();

    let h = grid.keys().map(|k| k.0).max().unwrap() + 1;
    let w = grid.keys().map(|k| k.1).max().unwrap() + 1;

    let cost_map = get_costs(grid, (0, 0), h, w, 4, 10);

    cost_map.get(&(h - 1, w - 1)).unwrap().unwrap()
}

fn solve_day_17_part1(input: &str) -> u32 {
    let grid: HashMap<(usize, usize), u32> = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(move |(j, c)| ((i, j), c.to_digit(10).unwrap()))
        })
        .collect();

    let h = grid.keys().map(|k| k.0).max().unwrap() + 1;
    let w = grid.keys().map(|k| k.1).max().unwrap() + 1;

    let cost_map = get_costs(grid, (0, 0), h, w, 0, 3);

    cost_map.get(&(h - 1, w - 1)).unwrap().unwrap()
}

fn get_costs(
    grid: HashMap<(usize, usize), u32>,
    starting_pos: (usize, usize),
    h: usize,
    w: usize,
    min_steps: u32,
    max_steps: u32,
) -> HashMap<(usize, usize), Option<u32>> {
    let mut steps: VecDeque<Step> = vec![
        Step::new(Direction::Down, starting_pos, 0),
        Step::new(Direction::Right, starting_pos, 0),
    ]
    .into();

    let mut block_map: HashMap<(usize, usize), Block> =
        grid.keys().map(|k| (k.clone(), Block::new())).collect();

    while let Some(s) = steps.pop_front() {
        let blk = block_map.get_mut(&s.position);
        if let Some(blk) = blk {
            if blk.update(&s.accumulated_cost, &s.direction) {
                let mut new_steps = s.generate_steps(&grid, w, h, min_steps, max_steps);
                new_steps = new_steps
                    .iter()
                    .filter(|ss| {
                        block_map
                            .get(&ss.position)
                            .map(|b| {
                                b.costs
                                    .get(&ss.direction)
                                    .map_or(true, |&b| b > ss.accumulated_cost)
                            })
                            .unwrap_or(false)
                    })
                    .map(|s| s.clone())
                    .collect();
                steps.append(&mut new_steps);
            }
        }
    }
    block_map
        .iter()
        .map(|(k, v)| (k.clone(), v.costs.values().min().map(|m| m.clone())))
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn move_dir(&self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Right => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0, pos.1 - 1),
        }
    }

    fn turn_directions(&self) -> Vec<Direction> {
        match self {
            Direction::Up | Direction::Down => vec![Direction::Right, Direction::Left],
            Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
        }
    }

    // fn get_steps(pos: (usize, usize), w: usize, h: usize, current_dir: Direction) {

    // }
}

#[derive(Debug, Clone)]
struct Step {
    direction: Direction,
    position: (usize, usize),
    accumulated_cost: u32,
}

impl Step {
    fn new(direction: Direction, position: (usize, usize), accumulated_cost: u32) -> Step {
        Step {
            direction,
            position,
            accumulated_cost,
        }
    }

    fn generate_steps(
        &self,
        grid: &HashMap<(usize, usize), u32>,
        w: usize,
        h: usize,
        min_steps: u32,
        max_steps: u32,
    ) -> VecDeque<Step> {
        let directions = self.direction.turn_directions();
        directions
            .iter()
            .flat_map(|d| {
                let mut acc_cost = self.accumulated_cost.clone();
                let mut pos = self.position;
                let mut steps = VecDeque::<Step>::new();
                // println!("testing pos {:?} with dir {:?} ", pos, d);
                let mut count_steps = 0;
                while let Some(p) = Self::valid_pos(&pos, d, &h, &w) {
                    pos.0 = p.0;
                    pos.1 = p.1;
                    acc_cost += grid.get(&pos).unwrap_or(&0);
                    let s = Step::new(d.clone(), pos.clone(), acc_cost.clone());
                    count_steps += 1;
                    if count_steps >= min_steps {
                        steps.push_front(s);
                    }

                    if count_steps >= max_steps {
                        break;
                    }
                }

                // println!("Generated Steps: {:?} ", steps);

                steps
            })
            .collect()
    }

    fn valid_pos(
        pos: &(usize, usize),
        direction: &Direction,
        h: &usize,
        w: &usize,
    ) -> Option<(usize, usize)> {
        let next_pos = direction.move_dir((pos.0 as i32, pos.1 as i32));

        if next_pos.0 >= 0
            && next_pos.0 < h.clone() as i32
            && next_pos.1 >= 0
            && next_pos.1 < w.clone() as i32
        {
            Some((next_pos.0 as usize, next_pos.1 as usize))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    costs: HashMap<Direction, u32>,
}

impl Block {
    fn new() -> Block {
        Block {
            costs: HashMap::<Direction, u32>::new(),
        }
    }

    fn update(&mut self, cost: &u32, dir: &Direction) -> bool {
        let current_cost = self.costs.get(&dir);

        if let Some(c) = current_cost {
            if cost.clone() >= c.clone() || c.clone() <= 0 {
                return false;
            }

            // println!("updating {:?} | {} -> {}", dir, c, cost);
        }
        //  else {
        //     println!("NEW {:?} | {}", dir, cost);
        // }
        self.costs.insert(dir.clone(), cost.clone());
        true
    }
}

#[cfg(test)]
const EXAMPLE_DATA_1: &str = include_str!("../test1.txt");

#[test]
fn example() {
    let part1 = solve_day_17_part1(EXAMPLE_DATA_1);
    assert_eq!(part1, 102);

    let part2 = solve_day_17_part2(EXAMPLE_DATA_1);
    assert_eq!(part2, 94);
}
