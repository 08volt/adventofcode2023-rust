use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    const FILE_PATH: &str = "./input.txt";

    let _numbers: [&str; 9] = [ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];


    let mut result = 0;

    if let Ok(lines) = read_lines(FILE_PATH) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {

                let a = first_digit(&line, _numbers);          

                let reversed_line: String = line.chars().rev().collect();  
                let reversed_numbers = _numbers.map(|n: &str| {
                    n.chars().rev().collect::<String>()
                });

                let b = first_digit(reversed_line, reversed_numbers);   
                
                match (a,b) {
                    (Some(a), Some(b)) => {
                        println!("{}{}", a, b);
                        let n = format!("{}{}", a, b).parse::<i32>().unwrap();
                        result = result + n;
                    },
                    (_,_) => ()
                }
                
                
            }
        }
    }
    println!("{}", result);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn first_digit(line: impl Into<String>, numbers: [impl Into<String>; 9]) -> Option<char>{
    let line: String = line.into();

    let mut smallest_index = line.chars().position(|c| c.is_digit(10)).unwrap_or_else(|| line.len());
    let mut smallest_res = line.chars().find(|c| c.is_digit(10));

    for (pos, number) in numbers.into_iter().enumerate() {        
        let i = line.find(&number.into());
        match i {
            Some(index) => {
                if index < smallest_index {
                    smallest_index = index;
                    smallest_res = std::char::from_digit((pos + 1).try_into().unwrap_or_default(), 10);
                }
            },
            None => (),
        }
    }
    
    smallest_res

}
