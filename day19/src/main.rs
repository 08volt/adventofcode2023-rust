use std::{
    collections::{HashMap, VecDeque},
    iter::zip,
    ops::Deref,
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
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

    // Benchmark
    println!("\nBenchmark:");
    let mut part1_v = Vec::<u128>::new();
    let mut part2_v = Vec::<u128>::new();
    for _ in 0..1000 {
        let now_1 = Instant::now();
        let part1 = solve_day_17_part1(INPUT);
        let elapsed_1 = now_1.elapsed();
        part1_v.push(elapsed_1.as_micros());
        let _ = part1;

        let now_2 = Instant::now();
        let part2 = solve_day_17_part2(INPUT);
        let elapsed_2 = now_2.elapsed();
        part2_v.push(elapsed_2.as_micros());
        let _ = part2;
    }
    println!(
        "Part 1 - Avarage: {:.2?}",
        part1_v.iter().sum::<u128>() / 1000 as u128
    );
    println!(
        "Part 2 - Avarage: {:.2?}",
        part2_v.iter().sum::<u128>() / 1000 as u128
    );
}

#[cfg(test)]
const EXAMPLE_DATA_1: &str = include_str!("../test1.txt");

#[test]
fn example() {
    let part1 = solve_day_17_part1(EXAMPLE_DATA_1);
    assert_eq!(part1, 19114);

    let part2 = solve_day_17_part2(EXAMPLE_DATA_1);
    assert_eq!(part2, 167409079868000);
}

fn solve_day_17_part2(input: &str) -> u128 {
    if let Some((workflows_str, _)) = input.split_once("\n\n") {
        let mut workflow: HashMap<Box<str>, Workflow> = workflows_str
            .lines()
            .map(|l| Workflow::from_str(l).unwrap())
            .map(|w| (w.name.clone(), w))
            .collect();
        workflow.remove("R");
        workflow.remove("A");
        let mut parts: VecDeque<FullPart> = vec![FullPart::new()].into();

        let mut accepted = Vec::<FullPart>::new();

        while let Some(p) = parts.pop_back() {
            if p.current_w.deref() == "A" {
                accepted.push(p)
            } else if p.current_w.deref() != "R" && p.is_valid() {
                let w = workflow.get(p.current_w.deref()).unwrap();
                let mut current_part = p.clone();
                for s in w.steps.iter() {
                    let new_parts = current_part.apply_step(s);
                    if let Some(new_p) = new_parts.1 {
                        parts.push_front(new_p);
                        if let Some(pp) = new_parts.0 {
                            current_part = pp;
                        }
                    } else {
                        if let Some(mut new_p) = new_parts.0 {
                            new_p.current_w = s.next.clone();
                            parts.push_front(new_p);
                        }
                    }
                }
            }
        }

        accepted
            .iter()
            .map(|a| {
                (a.ratings_max[0] - a.ratings_min[0] + 1) as u128
                    * (a.ratings_max[1] - a.ratings_min[1] + 1) as u128
                    * (a.ratings_max[2] - a.ratings_min[2] + 1) as u128
                    * (a.ratings_max[3] - a.ratings_min[3] + 1) as u128
            })
            .sum()
    } else {
        0
    }
}

fn solve_day_17_part1(input: &str) -> u32 {
    if let Some((workflows_str, parts_str)) = input.split_once("\n\n") {
        let workflow: HashMap<Box<str>, Workflow> = workflows_str
            .lines()
            .map(|l| Workflow::from_str(l).unwrap())
            .map(|w| (w.name.clone(), w))
            .collect();
        let mut parts: Vec<Part> = parts_str
            .lines()
            .map(|l| Part::from_str(l).unwrap())
            .collect();

        parts
            .iter_mut()
            .map(|p| {
                while p.current_w.deref() != "A" && p.current_w.deref() != "R" {
                    p.current_w = workflow.get(&p.current_w).unwrap().process_part(p).unwrap();
                }
                p
            })
            .filter(|p| p.current_w.deref() == "A")
            .map(|p| p.ratings.iter().sum::<u32>())
            .sum()
    } else {
        0
    }
}

struct Workflow {
    name: Box<str>,
    steps: Arc<[Step]>,
}

impl Workflow {
    fn rating_to_index(rating: char) -> Option<usize> {
        match rating {
            'x' => Some(0),
            'm' => Some(1),
            'a' => Some(2),
            's' => Some(3),
            _ => None,
        }
    }

    fn process_part(&self, part: &Part) -> Option<Box<str>> {
        for s in self.steps.iter() {
            if let Some(greater) = s.grater {
                if let Some(rating) = s.rating {
                    if let Some(value) = s.value {
                        let condition = match greater {
                            true => value < part.ratings[Self::rating_to_index(rating).unwrap()],
                            false => value > part.ratings[Self::rating_to_index(rating).unwrap()],
                        };
                        if condition {
                            return Some(s.next.clone());
                        }
                    }
                }
            } else {
                return Some(s.next.clone());
            }
        }
        None
    }
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((name, steps)) = s.split_once('{') {
            let steps = steps.to_string()[..steps.len() - 1].to_string();
            let steps: Arc<[Step]> = steps
                .split(',')
                .map(|s| Step::from_str(s).unwrap())
                .collect();
            Ok(Workflow {
                name: name.into(),
                steps,
            })
        } else {
            Err(())
        }
    }
}

struct Step {
    rating: Option<char>,
    value: Option<u32>,
    grater: Option<bool>,
    next: Box<str>,
}

impl FromStr for Step {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.split_once(':') {
            let mut c = a.chars();
            let rating = c.next();
            let grater = Some(c.next() == Some('>'));
            let value = c.as_str().parse::<u32>().ok();
            Ok(Step {
                rating,
                value,
                grater,
                next: b.into(),
            })
        } else {
            Ok(Step {
                rating: None,
                value: None,
                grater: None,
                next: s.into(),
            })
        }
    }
}

struct Part {
    ratings: Vec<u32>,
    current_w: Box<str>,
}

#[derive(Clone, Debug)]
struct FullPart {
    ratings_max: Vec<u32>,
    ratings_min: Vec<u32>,
    current_w: Box<str>,
}

impl FullPart {
    fn new() -> FullPart {
        FullPart {
            ratings_max: vec![4000; 4],
            ratings_min: vec![1; 4],
            current_w: "in".into(),
        }
    }

    fn is_valid(&self) -> bool {
        zip(self.ratings_max.iter(), self.ratings_min.iter()).all(|(r_max, r_min)| r_max >= r_min)
    }

    fn apply_step(&self, s: &Step) -> (Option<FullPart>, Option<FullPart>) {
        let mut part1 = self.clone();

        if let Some(greater) = s.grater {
            if let Some(rating) = s.rating {
                let index = Workflow::rating_to_index(rating).unwrap();
                if let Some(value) = s.value {
                    let mut part2 = self.clone();
                    let mut new_max = self.ratings_max.clone();
                    let mut new_min = self.ratings_min.clone();
                    part2.current_w = s.next.clone();
                    if greater {
                        // part2 rispetta la condizione
                        new_max[index] = value;
                        new_min[index] = value + 1;
                        part1.ratings_max = new_max;
                        part2.ratings_min = new_min;
                    } else {
                        new_max[index] = value - 1;
                        new_min[index] = value;
                        part2.ratings_max = new_max;
                        part1.ratings_min = new_min;
                    }
                    return (Some(part1), Some(part2));
                }
            }
        }
        return (Some(part1), None);
    }
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let ratrings_str: String = s[1..s.len() - 1].to_string();
        let ratings: Vec<u32> = ratrings_str
            .split(",")
            .map(|r| r.to_string().split_off(2))
            .map(|v| v.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        Ok(Part {
            ratings,
            current_w: "in".into(),
        })
    }
}
