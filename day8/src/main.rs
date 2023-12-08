use std::{str::{FromStr, Chars}, collections::HashMap};

fn main() {
    let input= include_str!("../input.txt");
    let part1 = solve_day_08(input, true);
    println!("{}", part1);
    let part2 = solve_day_08(input, false);
    println!("{}", part2);
}

#[derive(Debug, Clone)]
struct Node {
    value: String,
    left: String,
    right: String
}

#[derive(Debug)]
struct ParseNodeErr {}

impl FromStr for Node {
    type Err = ParseNodeErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("{}", s);
        let n = Node {
            value: s[..3].to_string(),
            left: s[7..10].to_string(),
            right: s[12..15].to_string(),
        };
        // println!("{:?}", n);
        return Ok(
            n
        )
    }
}

fn solve_day_08(input: &str, part1: bool) -> u128{
    let input: Vec<&str> = input.split("\n\n").collect();
    let instructions = (&input[0]).to_owned().chars();
    // println!("{:?}", instructions);


    let nodes:HashMap<String, Node> = 
    input[1]
    .lines()
    .map(|l| {
        (l, Node::from_str(l).unwrap())
    })
    .map(|(l, n)| (l[..3].to_owned(), n))
    .collect();

    // println!("{:?}", nodes);
    println!("{}", part1);

    match part1 {
        true => solve_part1(nodes, instructions) as u128,
        false => solve_part2(nodes, instructions),
    }
}

fn solve_part1(nodes: HashMap<String, Node>, instructions: Chars) -> u32 {
    let mut current_node: &Node = nodes.get("AAA").unwrap();
    let mut passi = 0;
    let mut instr_copy = instructions.clone();
    while current_node.value != "ZZZ" {
        let mut current_instr = instr_copy.next();
        // println!("{}", current_node.value);
        if current_instr == None {
            instr_copy = instructions.clone();
            current_instr = instr_copy.next();
        }
        let current_node_value: &str = if current_instr.unwrap() == 'R'  { &current_node.right } else { &current_node.left };
        current_node = nodes.get(current_node_value).unwrap();
        passi += 1
    }

    passi
}


fn steps_part2(nodes: &HashMap<String, Node>, instructions: &Chars, starting_node: &Node) -> u32  {
    let mut passi = 0;
    let mut instr_copy = instructions.clone();
    let mut current_node = starting_node;
    while !current_node.value.ends_with('Z') {
        let mut current_instr = instr_copy.next();
        // println!("{}", current_node.value);
        if current_instr == None {
            instr_copy = instructions.clone();
            current_instr = instr_copy.next();
        }
        let current_node_value: &str = if current_instr.unwrap() == 'R'  { &current_node.right } else { &current_node.left };
        current_node = nodes.get(current_node_value).unwrap();
        passi += 1
    }

    passi
}

fn solve_part2(nodes: HashMap<String, Node>, instructions: Chars) -> u128 {
    println!("Solving part 2\n\n{:?}\n\n", nodes);
    let starting_nodes: Vec<&Node> = nodes.iter().filter(|(s,_)| s.ends_with('A') ).map(|(_,n)| n).collect();

    println!("{:?}", starting_nodes);

    let steps: Vec<u128> = starting_nodes.iter().map(|&n| steps_part2(&nodes, &instructions, n) as u128).collect();

    println!("{:?}", steps);

    steps.iter().fold(1, |a ,b|  lcm(&a,b))
}

fn lcm(first: &u128, second: &u128) -> u128 {
    first * second / gcd(first, second)
}

fn gcd(first: &u128, second: &u128) -> u128 {
    let mut max:u128 = first.clone();
    let mut min:u128 = second.clone();
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

    // BRUTE FORCE
    // let mut passi = 0;
    // let mut instr_copy = instructions.clone();

    // while !current_nodes.iter().all(|n| n.value.ends_with('Z')) {
    //     let mut current_instr = instr_copy.next();

    //     if current_instr == None {
    //         instr_copy = instructions.clone();
    //         current_instr = instr_copy.next();
    //     }
    //     current_nodes = current_nodes.iter().map(|&n| {
    //         nodes.get(
    //             if current_instr.unwrap() == 'R'  { &n.right } else { &n.left }
    //         ).unwrap()
    //     } ).collect();
    //     println!("{}", passi);
    //     passi += 1
        
    // }





#[cfg(test)]
const EXAMPLE_DATA_1: &str = include_str!("../test1.txt");
#[cfg(test)]
const EXAMPLE_DATA_2: &str = include_str!("../test2.txt");
#[cfg(test)]
const EXAMPLE_DATA_3: &str = include_str!("../test3.txt");

#[test]
fn example() {
    let part1 = solve_day_08(EXAMPLE_DATA_1, true);
    assert_eq!(part1, 2);
    let part1 = solve_day_08(EXAMPLE_DATA_2, true);
    assert_eq!(part1, 6);

    let part2 = solve_day_08(EXAMPLE_DATA_3, false);
    assert_eq!(part2, 6);
}
