use std::{
    collections::{HashMap, VecDeque},
    ops::Deref,
    str::FromStr,
    time::Instant,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now_1 = Instant::now();
    let part1 = solve_day_20_part1(INPUT);
    let elapsed_1 = now_1.elapsed();
    println!("part 1 - Result -> {}", part1);
    println!("Part 1 - Elapsed: {:.2?}", elapsed_1);

    let now_2 = Instant::now();
    let part2 = solve_day_20_part2(INPUT);
    let elapsed_2 = now_2.elapsed();
    println!("part2 - Result -> {}", part2);
    println!("Part 2 - Elapsed: {:.2?}", elapsed_2);

    // Benchmark
    println!("\nBenchmark:");
    let mut part1_v = Vec::<u128>::new();
    let mut part2_v = Vec::<u128>::new();
    for _ in 0..1000 {
        let now_1 = Instant::now();
        let part1 = solve_day_20_part1(INPUT);
        let elapsed_1 = now_1.elapsed();
        part1_v.push(elapsed_1.as_micros());
        let _ = part1;

        let now_2 = Instant::now();
        let part2 = solve_day_20_part2(INPUT);
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

#[cfg(test)]
const EXAMPLE_DATA_2: &str = include_str!("../test2.txt");

#[test]
fn example() {
    let part1 = solve_day_20_part1(EXAMPLE_DATA_1);
    assert_eq!(part1, 32000000);

    let part1 = solve_day_20_part1(EXAMPLE_DATA_2);
    assert_eq!(part1, 11687500);

    //     let part2 = solve_day_20_part2(EXAMPLE_DATA_1);
    //     assert_eq!(part2, 167409079868000);
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Level {
    HIGH,
    LOW,
}

#[derive(Debug, Clone)]
struct Pulse {
    from: Box<str>,
    level: Level,
    destination: Box<str>,
}

trait Module {
    fn receive_pulse(&mut self, inn: &Pulse) -> Vec<Pulse>;
}

struct Broadcast {
    name: Box<str>,
    dest: Vec<Box<str>>,
}

impl Into<String> for &Broadcast {
    fn into(self) -> String {
        format!("broadcast {} | destination {:?}", self.name, self.dest)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum State {
    ON,
    OFF,
}

struct FlipFlop {
    name: Box<str>,
    dest: Vec<Box<str>>,
    state: State,
}

impl Into<String> for &FlipFlop {
    fn into(self) -> String {
        format!(
            "flipflop {} | destination {:?} | state: {:?} ",
            self.name, self.dest, self.state
        )
    }
}

struct Conjunction {
    name: Box<str>,
    dest: Vec<Box<str>>,
    memory: HashMap<Box<str>, Level>,
}

impl Into<String> for &Conjunction {
    fn into(self) -> String {
        format!(
            "conjunction {} | destination {:?} | memory: {:?} ",
            self.name, self.dest, self.memory
        )
    }
}

impl Module for Broadcast {
    fn receive_pulse(&mut self, inn: &Pulse) -> Vec<Pulse> {
        self.dest
            .iter()
            .map(|d| Pulse {
                from: self.name.clone(),
                level: inn.level.clone(),
                destination: d.clone(),
            })
            .collect()
    }
}

impl Module for FlipFlop {
    fn receive_pulse(&mut self, inn: &Pulse) -> Vec<Pulse> {
        if inn.level == Level::LOW {
            let level = match self.state {
                State::ON => Level::LOW,
                State::OFF => Level::HIGH,
            };
            self.state = match self.state {
                State::ON => State::OFF,
                State::OFF => State::ON,
            };
            self.dest
                .iter()
                .map(|d| Pulse {
                    from: self.name.clone(),
                    level: level.clone(),
                    destination: d.clone(),
                })
                .collect()
        } else {
            Vec::<Pulse>::new()
        }
    }
}

impl Module for Conjunction {
    fn receive_pulse(&mut self, inn: &Pulse) -> Vec<Pulse> {
        self.memory.insert(inn.from.clone(), inn.level.clone());
        let level = match self.memory.values().all(|l| l.clone() == Level::HIGH) {
            true => Level::LOW,
            false => Level::HIGH,
        };
        self.dest
            .iter()
            .map(|d| Pulse {
                from: self.name.clone(),
                level: level.clone(),
                destination: d.clone(),
            })
            .collect()
    }
}

struct MachineSet {
    modules: HashMap<Box<str>, Box<dyn Module>>,
}

impl FromStr for MachineSet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let modules_list: Vec<&str> = s.lines().map(|l| l.trim()).collect();

        let conj_list: Vec<&str> = modules_list
            .iter()
            .filter(|m| m.starts_with("&"))
            .map(|&s| s)
            .collect();

        let mut conjs: HashMap<Box<str>, Box<Conjunction>> = conj_list
            .iter()
            .map(|f_str| {
                let (name_str, dest_str) = f_str.split_once(" -> ").unwrap();
                let name: Box<str> = name_str.to_string()[1..].into();
                let dest: Vec<Box<str>> = dest_str.split(",").map(|n| n.trim().into()).collect();
                (
                    name.clone(),
                    Box::new(Conjunction {
                        name,
                        dest,
                        memory: HashMap::<Box<str>, Level>::new(),
                    }),
                )
            })
            .collect();

        let conj_connections: Vec<(Box<str>, Box<str>)> = conjs
            .values()
            .flat_map(|c| {
                conjs
                    .values()
                    .filter(|c| c.dest.contains(&c.name))
                    .map(|coming| (c.name.clone(), coming.name.clone()))
            })
            .collect();

        conj_connections.iter().for_each(|(c, inn)| {
            conjs
                .get_mut(c)
                .unwrap()
                .memory
                .insert(inn.clone(), Level::LOW);
        });

        let mut modules = HashMap::<Box<str>, Box<dyn Module>>::new();

        let (_, dest_str) = modules_list
            .iter()
            .find(|&m| m.starts_with("broadcaster"))
            .ok_or(())?
            .split_once(" -> ")
            .ok_or(())?;

        let dest: Vec<Box<str>> = dest_str.split(",").map(|n| n.trim().into()).collect();

        dest.iter().for_each(|d| {
            conjs.get_mut(d).map(|c| {
                c.memory.insert("broadcaster".into(), Level::LOW);
            });
        });

        modules.insert(
            "broadcaster".into(),
            Box::new(Broadcast {
                name: "broadcaster".into(),
                dest,
            }),
        );

        let ffs_list: Vec<&str> = modules_list
            .iter()
            .filter(|m| m.starts_with("%"))
            .map(|&s| s)
            .collect();

        ffs_list.iter().for_each(|f_str| {
            let (name_str, dest_str) = f_str.split_once(" -> ").unwrap();
            let name: Box<str> = name_str.to_string()[1..].into();
            let dest: Vec<Box<str>> = dest_str.split(",").map(|n| n.trim().into()).collect();
            dest.iter().for_each(|d| {
                conjs.get_mut(d).map(|c| {
                    c.memory.insert(name.clone(), Level::LOW);
                });
            });
            modules.insert(
                name.clone(),
                Box::new(FlipFlop {
                    name: name,
                    dest,
                    state: State::OFF,
                }),
            );
        });

        for c in conjs.into_values() {
            modules.insert(c.name.clone(), c);
        }

        Ok(MachineSet { modules })
    }
}

impl MachineSet {
    fn press_button(&mut self, memory: &mut HashMap<Box<str>, u128>, i: &u128) -> (u128, u128) {
        let mut pulses_sent: (u128, u128) = (0, 1);
        let mut pulses_to_process = VecDeque::<Pulse>::new();

        let first_pulse = Pulse {
            from: "button".into(),
            level: Level::LOW,
            destination: "broadcaster".into(),
        };

        pulses_to_process.push_back(first_pulse);

        while let Some(p) = pulses_to_process.pop_front() {
            // println!(" {}  -{:?}-> {} ", p.from, p.level, p.destination);
            if let Some(m) = self.modules.get_mut(p.destination.deref()) {
                let new_pulses: Vec<Pulse> = m.receive_pulse(&p);
                if p.destination.deref() == "jq" && p.level == Level::HIGH {
                    memory.insert(p.from.clone(), i.clone());
                };

                new_pulses
                    .iter()
                    .for_each(|p| pulses_to_process.push_back(p.clone()));

                pulses_sent = new_pulses.iter().fold(pulses_sent, |p, n| match n.level {
                    Level::HIGH => (p.0 + 1, p.1),
                    Level::LOW => (p.0, p.1 + 1),
                });
            }

            // let m_str: String = (&self.clone()).into();
            // println!("{}", m_str);
            // println!("\n{} pulses to process", pulses_to_process.len())
        }

        pulses_sent
    }
}

fn solve_day_20_part2(input: &str) -> u128 {
    let mut machines = MachineSet::from_str(input).unwrap();

    let mut cnt: u128 = 0;
    let mut memory = HashMap::<Box<str>, u128>::new();
    while memory.len() < 4 {
        cnt += 1;
        machines.press_button(&mut memory, &cnt);
    }

    memory
        .values()
        .fold(memory.values().next().unwrap().clone(), |x, y| {
            lcm(x, y.clone())
        })
}

fn lcm(first: u128, second: u128) -> u128 {
    first * second / gcd(first, second)
}

fn gcd(first: u128, second: u128) -> u128 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn solve_day_20_part1(input: &str) -> u128 {
    let mut machines = MachineSet::from_str(input).unwrap();

    let mut memory = HashMap::<Box<str>, u128>::new();

    let mut res = (0, 0);
    for i in 0..1000 {
        let tmp = machines.press_button(&mut memory, &i);
        res.0 += tmp.0;
        res.1 += tmp.1;
    }
    res.0 * res.1
}
