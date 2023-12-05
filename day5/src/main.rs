use std::{ str::FromStr, path::Path, io::{self, Read}, fs::File};

fn main() {
    const FILE_PATH: &str = "./input.txt";

    let Ok(lines) = read_file(FILE_PATH) else {
        return
    };

    let lines = "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4";

    let lines:Vec<&str> = lines.split("\n").map(|l| l.trim()).collect();
    let lines = lines.join("\n");

    let ls:Vec<&str> = lines.split("\n\n").collect();
    let seeds: Vec<i64> = ls.first().unwrap()[7 ..].trim().split(" ").map(|seed| seed.parse::<i64>().unwrap()).collect();


    let maps: Vec<Map> = ls[1..].iter().map(|&m| Map::from_str(m).unwrap()).filter(|v| v.maps.len() > 0).collect();
    let mut min_location: Option<i64> = None;

    

    for s in seeds {
        println!("seed {} ", s);

        let location = maps.iter().fold(s, | a, b | b.convert(a) );
        println!("location {} ", location);
        min_location = match min_location {
            Some(l) if l > location => Some(location),
            None => Some(location),
            Some(_) => min_location,
        }  
    }

    println!("{}", min_location.unwrap());

}

#[derive(Debug )]
struct Range {
    dest_start:i64,
    source_start: i64,
    rang_len: i64
}

impl Range {
    fn convert(&self, value: i64) -> Option<i64>{
        println!("v: {} | range {:?}", value, self);

        if self.source_start <= value && (self.source_start + self.rang_len) > value {
            
            println!("{}", value - self.source_start + self.dest_start);
            Some(value - self.source_start + self.dest_start)
        } else {
            None
        }
    }
}

#[derive(Debug )]
struct Map {
    maps: Vec<Range>
}

impl Map {
    fn convert(&self, value: i64) -> i64 {
        for m in &self.maps {
            
            if let Some(res) = m.convert(value) {
                println!("{}", res);
                return res;
            }
        }
        return value.clone();
    }
}

#[derive(Debug )]
struct ParseMapError {}

impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut maps :Vec<Range> = Vec::<Range>::new();
        for l in s.lines() {
            if l.starts_with(|x: char| x.is_numeric()) {
                let r = Range::from_str(l).map_err(|_| ParseMapError {})?;
                maps.push(r); 
            }
        }
        Ok(
            Map {
                maps,
            }
        )
    }
}
#[derive(Debug )]
struct ParseRangeError {}

impl FromStr for Range {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<&str> = s.trim().split(" ").collect();
        if values.len() != 3 {
            return Err(ParseRangeError {});
        }

        return Ok(
            Range {
                dest_start: values[0].parse::<i64>().map_err(|_| ParseRangeError {})?,
                source_start: values[1].parse::<i64>().map_err(|_| ParseRangeError {})?,
                rang_len: values[2].parse::<i64>().map_err(|_| ParseRangeError {})?
            }
        )


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