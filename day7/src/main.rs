use std::{str::FromStr, hash::Hash, collections::HashSet};

fn main() {
    let input = include_str!("../input.txt");
    let part1 = solve_day_07(input);
    println!("{}", part1);
}


fn solve_day_07(input: &str) -> u32{
    let mut hands: Vec<Hand> = input.lines().map(|l| Hand::from_str(l).unwrap()).collect();
    hands.sort();
    for h in &hands {
        println!("{:?}", h)
    }
    let res = hands.iter().enumerate().map(|(i, h)| ((i+1) as u32) * h.bid).fold(0, |a,b| a + b );
    res
}



#[cfg(test)]
const EXAMPLE_DATA: &str = include_str!("../test.txt");

#[test]
fn example() {
    let part2 = solve_day_07(EXAMPLE_DATA);
    //assert_eq!(part1, 6440);
    assert_eq!(part2, 5905);
}

#[derive(Clone, Eq, Debug, Hash)]
struct Card {
    symbol: char
}

impl Card {
    fn new(symbol: char) -> Result<Card, String>{
        if ['2','3','4','5','6','7','8','9','T','J','Q','K','A'].contains(&symbol) {
            Ok(Card {
                symbol,
            })
        } else {
            Err(
                "Not valid".to_owned()
            )
        }
    }

    fn value(&self) -> Option<u32> {
        match self.symbol {
            'J' => Some(1),
            s if s.is_numeric() => {
                Some(s.to_digit(10).unwrap())
            }
            'A' => Some(14),
            'T' => Some(10),
            'Q' => Some(12),
            'K' => Some(13),
            _ => None
        }
    }
    
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.type_value() == other.type_value() {
            self.cards.cmp(&other.cards)
        } else {
            self.type_value().cmp(&other.type_value())
        }
    }
}

impl Hand {
    fn type_value(&self) -> i32 {
        let mut distinct_values:HashSet<&Card> = self.cards.iter().filter(|&c| c.symbol != 'J').collect();
        if distinct_values.is_empty() {
            distinct_values.insert(&Card { symbol: 'J' });
        }
        let m: usize = 
        distinct_values
        .iter()
        .map(|&v| self.cards.iter().filter(|&c| c == v).count() )
        .max()
        .unwrap() + self.cards.iter().filter(|&c| c.symbol == 'J').count();
        
        match distinct_values.len() {
            1 => {
                6
            },
            2 => {
                // 3|2 or 4|1
                if m == 4 {
                    5
                } else {
                    4
                }
            },
            3 => {
                // 2|2|1 or 3|1|1
                if m == 3 {
                    3
                } else {
                    2
                }
            },
            4 => {
                1
            },
            _ => {
                0
            },
        }

    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.type_value() == other.type_value() {
            self.cards.partial_cmp(&other.cards)
        } else {
            self.type_value().partial_cmp(&other.type_value())
        }
    }
}

impl Eq for Hand {
    
}

#[derive(Debug)]
struct ParseHandeErr {}

impl FromStr for Hand {
    type Err = ParseHandeErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.split(" ").collect();
        let cards: Result<Vec<Card>, String> = s[0].chars().map(|c| Card::new(c)).collect();
        let cards = cards.map_err(|e| ParseHandeErr {})?;
        let bid = s[1].parse::<u32>().unwrap();
        Ok(Hand {
            cards,
            bid
        })
    }
}