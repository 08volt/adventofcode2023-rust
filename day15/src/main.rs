use std::{
    collections::VecDeque,
    fmt::{self},
    time::Instant,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now_1 = Instant::now();
    let part1 = solve_day_15_part1(INPUT);

    println!("part 1 - Result -> {}", part1);

    let elapsed_1 = now_1.elapsed();
    println!("Part 1 - Elapsed: {:.2?}", elapsed_1);

    let now_2 = Instant::now();
    let part2 = solve_day_15_part2(INPUT);

    println!("part2 - Result -> {}", part2);

    let elapsed_2 = now_2.elapsed();
    println!("Part 2 - Elapsed: {:.2?}", elapsed_2);
}

fn solve_day_15_part2(input: &str) -> u64 {
    let mut boxes = vec![LensBox::new(); 256];
    for step in input.split(',') {
        let label = step[0..step
            .chars()
            .position(|c| c == '=')
            .unwrap_or(step.len() - 1)]
            .to_owned();

        // println!("\n\n{}", label);

        match step.chars().last().unwrap() {
            '-' => {
                let cur_box = boxes.get_mut(string_hash(label.as_str())).unwrap();
                cur_box.remove(label);
            }
            _ => {
                let focal_lenght = step.split('=').last().unwrap().parse::<u64>().unwrap();
                let new = Lens {
                    label,
                    focal_lenght,
                };
                let cur_box = boxes.get_mut(new.box_index()).unwrap();
                cur_box.insert(new);
            }
        }
        // for (i, b) in boxes.iter().enumerate() {
        //     if b.lens.len() > 0 {
        //         print!("{} |  {}\n", i, b);
        //     }
        // }
    }

    boxes.iter().enumerate().fold(0 as u64, |s, (j, b)| {
        s + b
            .lens
            .iter()
            .enumerate()
            .map(|(i, l)| {
                // println!(" {} {} {} ", (1 + j as u64), (1 + i as u64), l.focal_lenght);
                (1 + j as u64) * (1 + i as u64) * l.focal_lenght
            })
            .sum::<u64>()
    })
}

fn solve_day_15_part1(input: &str) -> u64 {
    input.split(',').map(|s| string_hash(s) as u64).sum()
}

fn step(current_value: u64, c: char) -> u64 {
    (((c as u64) + current_value) * 17) % 256
}

fn string_hash(s: &str) -> usize {
    s.chars().fold(0 as u64, |v, c| step(v, c)) as usize
}

#[derive(Debug, Clone)]
struct LensBox {
    lens: VecDeque<Lens>,
}

impl fmt::Display for LensBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.lens.len() == 0 {
            write!(f, "")
        } else {
            write!(f, "{:?}", self.lens)
        }
    }
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_lenght: u64,
}

impl Lens {
    fn box_index(&self) -> usize {
        string_hash(&self.label.as_str()) as usize
    }
}

impl LensBox {
    fn new() -> LensBox {
        LensBox {
            lens: VecDeque::<Lens>::new(),
        }
    }

    fn insert(&mut self, new: Lens) {
        match self.lens.iter().position(|l| l.label == new.label) {
            Some(i) => {
                self.lens.remove(i);
                self.lens.insert(i, new)
            }
            None => self.lens.push_back(new),
        }
    }

    fn remove(&mut self, label: String) {
        if let Some(i) = self.lens.iter().position(|x| x.label == label) {
            self.lens.remove(i);
        }
    }
}

#[cfg(test)]
const EXAMPLE_DATA_1: &str = include_str!("../test1.txt");

#[test]
fn example() {
    let part1 = solve_day_15_part1(EXAMPLE_DATA_1);
    assert_eq!(part1, 1320);

    let part2 = solve_day_15_part2(EXAMPLE_DATA_1);
    assert_eq!(part2, 145);
}
