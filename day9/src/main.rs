use std::{fs::read_to_string, str::FromStr};

fn main() {
    if let Ok(input) = read_to_string("input.txt") {

        let total: (i128, i128) = input
        .lines()
        .filter_map(|l| l.parse::<History>().ok())
        .map(|h| (h.last(), h.first()))
        .fold((0, 0), | (a,b), (c,d) | (a+c, b+d));

        println!("{:?}", total)
    }
}


struct History {
    sequence: Vec<i128>
}

impl History {
    fn last(&self) -> i128 {
        Self::add_to_sequence(&self.sequence)
    }

    fn first(&self) -> i128 {
        Self::add_to_left(&self.sequence)
    }

    fn add_to_left(sequence: &Vec<i128>) -> i128 {
        if sequence.len() == 1 || sequence.windows(2).all(|w| w[0] == w[1]) {            
            return sequence.first().unwrap_or(&0).clone();
        }

        let prev = Self::add_to_left(&Self::create_diff_sequence(sequence));

        return sequence.first().unwrap_or(&0) - prev;
    }

    fn add_to_sequence(sequence: &Vec<i128>) ->  i128 {
        if sequence.len() == 1 || sequence.windows(2).all(|w| w[0] == w[1]) {            
            return sequence.first().unwrap_or(&0).clone();
        }

        let prev = Self::add_to_sequence(&Self::create_diff_sequence(sequence));

        return sequence.last().unwrap_or(&0) + prev;
    }

    fn create_diff_sequence(sequence: &Vec<i128>) ->  Vec<i128> {
        return sequence.windows(2).map(|w| w[1] - w[0]).collect();
    }
}

#[derive(Debug)]
struct HistoryParseError {}

impl FromStr for History {
    type Err = HistoryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seq: Vec<i128> = s.trim().split(" ").filter_map(|n| n.parse::<i128>().ok()).collect();
        Ok(History {
            sequence: seq,
        })
    }
}