use std::{collections::HashSet, str::FromStr, num::ParseIntError, path::Path, io::{self, Read}, fs::File};

fn main() {
    const FILE_PATH: &str = "./input.txt";

    let Ok(lines) = read_file(FILE_PATH) else {
        return
    };

    // let lines = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
 
    let cards: Result<Vec<Card>, ParseCardError> = lines.lines().map(|s| Card::from_str(s)).collect();

    let mut cards = cards.unwrap();
    let points: Vec<i32> = cards.iter().map(|c| c.points()).collect();
    let n_cards = cards.len();
    for i in 0 .. n_cards {
        let matches = cards[i].winning_numers().len();
        if matches == 0 {
            continue;
        }
        for index in i+1 .. i+matches+1 {
            cards[index].copies += cards[i].copies;
        }
    }

    let s: i32 = points.iter().sum();
    println!("{}", s);

    let cards_stack: i32 = cards.iter().map(|c| c.copies).sum();
    println!("{}", cards_stack);
}

#[derive(Debug )]
struct Card {
    winning_set : HashSet<i32>,
    numbers : HashSet<i32>,
    copies: i32,
    card_id: i32
}

impl Card {
    fn new(id: i32) -> Card {
        Card {
            winning_set: HashSet::new(),
            numbers: HashSet::new(),
            copies: 1,
            card_id: id
        }
    }

    fn _id(&self) -> i32{
        self.card_id
    }

    fn winning_numers(&self) -> HashSet<i32>{
        self.numbers.intersection(&self.winning_set).map(|&n| n).collect()
    }

    fn points(&self) -> i32 {
        let p = self.winning_numers().len();
        match p {
            0 => 0,
            p => (2 as i32).pow(p as u32 - 1)
        }
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

#[derive(Debug, PartialEq, Eq)]
struct ParseCardError;


impl From<ParseIntError> for ParseCardError{
    fn from(_: ParseIntError) -> Self {
        ParseCardError
    }
}

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let halfs: Vec<&str> = s.split(':').collect();
        if halfs.len() != 2 {
            println!("halfs: {:?}", s);
            return Err(ParseCardError);
        }

        let id_str: Vec<&str> = halfs[0].split(" ").filter(|&s| s.len() > 0).collect();
        
        let id_str: String = id_str[1].trim().to_string();

        let id = id_str.trim().parse::<i32>()?;

        let lists: Vec<&str> = halfs[1].split("|").collect();

        if lists.len() != 2 {
            println!("lists: {}", halfs.len());
            return Err(ParseCardError);
        }

        let mut c = Card::new(id);

        c.winning_set = lists[0].split(" ").filter_map(|n| n.trim().parse::<i32>().ok()).collect();
        c.numbers = lists[1].split(" ").filter_map(|n| n.trim().parse::<i32>().ok()).collect();

        Ok(c)        
    }
}