use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    const FILE_PATH: &str = "./input.txt";

    let Ok(lines) = read_lines(FILE_PATH) else {
        return
    };

    let mut result_1 = 0;
    let mut result_2 = 0;

    for line in lines {
        let Ok(line) = line else {
            return
        };
        let parts: Vec<&str> = line.split(":").collect();

        // Checking if the split was successful
        if parts.len() == 2 {

            let game =  parts[0][5 ..].parse::<i32>().unwrap_or_default();
            let set_string = parts[1];

            let sets: Vec<&str> = set_string.split(';').flat_map(|ex: &str| ex.split(',').collect::<Vec::<&str>>()).map(|x: &str| x.trim()).collect();
            
            if sets.iter().all(|&set| isValidSet(set)) {
                result_1 += game;
            }

            let mut min_color: HashMap<_, _> = HashMap::new();

            for set in sets {
                let parts: Vec<&str> = set.split(" ").collect();
                if parts.len() == 2 {
                    let amount =  parts[0].parse::<i32>().unwrap_or_default();
                    let color = parts[1];

                    match min_color.get(color) {
                        Some(&old_amount) => {
                            if old_amount < amount {
                                min_color.insert(color, amount);
                            }
                        },
                        None => {
                            min_color.insert(color, amount);
                        },
                    }

                }
            
            }
            let tot = min_color.iter().fold(1, |acc, (_, &c)| acc * c);
            
            result_2 += tot;
        }
    
    }

    println!("{}", result_1);
    println!("{}", result_2);

}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn isValidSet(set: impl Into<String>) -> bool{
    let set = set.into();
    let parts: Vec<&str> = set.split(" ").collect();
    let mut color_amount_map = HashMap::new();

    // Inserting color and amount pairs into the HashMap
    color_amount_map.insert(String::from("red"), 12);
    color_amount_map.insert(String::from("green"), 13);
    color_amount_map.insert(String::from("blue"), 14);

    // Checking if the split was successful
    if parts.len() == 2 {
        let amount =  parts[0].parse::<i32>().unwrap_or_default();
        let color = parts[1];

        let Some(max) = color_amount_map.get(color) else {
            return false;
        };
        if max < &amount {
            return false;
        }
    }
    return true
}
