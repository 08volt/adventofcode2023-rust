use std::iter::Sum;
use std::{collections::HashSet, io::Read, cmp::max};
use std::fs::File;
use std::io;
use std::path::Path;


fn main() {
    const FILE_PATH: &str = "./input.txt";

    let Ok(lines) = read_file(FILE_PATH) else {
        return
    };

    // let lines = "*467.114..
    // ..........
    // ..35..633.
    // ......#...
    // 617.3.....
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$......
    // .664.598..";

    let symbols: Vec<Symbol> = find_symbol_positions(&lines);


    let numbers: Vec<Number>  = find_number_positions(&lines);


    let valid_numbers: Vec<i32> = numbers
    .iter()
    .filter(| &n | { n.is_valid_number(&(symbols.iter().map(|s| (s.line, s.col)).collect()))})
    .map(|n |  { n.value })
    .collect();

    let sum: i32 = valid_numbers.iter().sum();
    println!("the total sum is: {}", sum);

    let gears: Vec<i64> = symbols
    .iter()
    .filter(|s| s.value == '*' && s.adjacent_numbers(&numbers).len() == 2)
    .map(|s| s.adjacent_numbers(&numbers).iter().fold(1 as i64, |a, b| {a * (b.value as i64)}))
    .collect();
    let sum: i64= gears.iter().sum();
    println!("the total gear is: {}", sum);
    
}

#[derive(Debug, Clone)]
struct Number {
    value: i32,
    line_index: i32,
    start_index: i32,
    end_index: i32
}

impl Number {
    fn area(&self) -> HashSet<(i32,i32)> {
        let min_line = max(self.line_index - 1, 0);
        let max_line = self.line_index + 1;

        let min_col = max(self.start_index - 1, 0);
        let max_col = self.end_index + 1;

        (min_line .. max_line+1).flat_map(| line| {
            
            (min_col .. max_col).map(move |col| (line, col))
        }).collect()

    }
    fn is_valid_number(&self, symbols: &HashSet<(i32, i32)>) -> bool{
        symbols.intersection(&self.area()).count() > 0
    }   
}

impl Into<i32> for Number {
    fn into(self) -> i32 {
        self.value
    }
}

#[derive(Debug)]
struct Symbol {
    value: char,
    line: i32,
    col: i32
}

impl Symbol {
    fn adjacent_numbers(&self, numbers: &Vec<Number>) -> Vec<Number>{
        numbers
        .iter()
        .filter(|&n| n.area().contains(&(self.line, self.col)))
        .map(|n| n.clone())
        .collect()
    }
}


fn read_file<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn find_symbol_positions(input: &str) -> Vec<Symbol> {
    input.lines()
    .enumerate()
    .flat_map(|(i, line)| {
        let symbols : Vec<Symbol> = line.trim().chars().enumerate()
        .filter(|(_, c)| {
            !c.is_alphanumeric() && *c != '.' 
        })
        .map(|(index, c)| Symbol { value: c, line: i as i32, col: index as i32 }).collect();
        symbols
    }).collect()
}

fn find_number_positions(input: &str) -> Vec<Number> {

    let mut number_positions:Vec<Number> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let l = line.trim();
        let mut current = String::new();
        let mut start = None;
        for (index, character) in l.chars().enumerate() {
            if character.is_numeric() {
                if start == None {
                    start = Some(index)
                }
                current.push(character);
            } else if let Some(s) = start {
                
                number_positions.push(Number {
                    value: current.parse().unwrap(), 
                    line_index: i as i32,
                    start_index: s as i32,
                    end_index: index as i32,
                });
                start = None;
                current = String::new();
            }
        }
        if current.len() > 0 {
            number_positions.push(Number {
                value: current.parse().unwrap(), 
                line_index: i as i32,
                start_index: start.unwrap() as i32,
                end_index: l.len() as i32,
            });
        }
    }
    number_positions
}

