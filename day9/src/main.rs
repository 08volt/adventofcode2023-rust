use std::fs::read_to_string;

fn main() {
    if let Ok(input) = read_to_string("input.txt") {
        let total: (i128, i128) = input
            .lines()
            .filter_map(|l| <Vec<i128> as History<i128>>::from_str(l).ok())
            .map(|h| (h.after_last(), h.before_first()))
            .fold((0, 0), |(a, b), (c, d)| (a + c, b + d));

        println!("{:?}", total)
    }
}
pub trait History<T> {
    fn after_last(&self) -> T;
    fn before_first(&self) -> T;
    fn create_diff_sequence(&self) -> Vec<T>;
    fn from_str(s: &str) -> Result<Vec<T>, String>;
}

impl History<i128> for Vec<i128> {
    fn after_last(&self) -> i128 {
        match self.len() {
            0 => 0,
            1 => self.first().unwrap_or(&0).clone(),
            _ if self.windows(2).all(|w| w[0] == w[1]) => self.first().unwrap_or(&0).clone(),
            _ => Self::after_last(&Self::create_diff_sequence(self)) + self.last().unwrap_or(&0),
        }
    }
    fn before_first(&self) -> i128 {
        match self.len() {
            0 => 0,
            1 => self.first().unwrap_or(&0).clone(),
            _ if self.windows(2).all(|w| w[0] == w[1]) => self.first().unwrap_or(&0).clone(),
            _ => self.first().unwrap_or(&0) - Self::before_first(&Self::create_diff_sequence(self)),
        }
    }

    fn create_diff_sequence(&self) -> Vec<i128> {
        self.windows(2).map(|w| w[1] - w[0]).collect()
    }

    fn from_str(s: &str) -> Result<Vec<i128>, String> {
        let seq: Vec<i128> = s
            .trim()
            .split(" ")
            .filter_map(|n| n.parse::<i128>().ok())
            .collect();
        Ok(seq)
    }
}
