use std::{ path::Path, io::{self, Read}, fs::File, iter::zip};


fn main() {
    let input = read_file("./input.txt").unwrap();

    let lines: Vec<&str> = input.split("\n").collect();

    let times: Vec<u64> = lines[0][5..].split(" ").filter(|&s| s.len() > 0).map(|n| n.trim().parse::<u64>().unwrap()).collect();
    let distances: Vec<u64> = lines[1][9..].split(" ").filter(|&s| s.len() > 0).map(|n| n.trim().parse::<u64>().unwrap()).collect();

    let result = zip(times, distances)
    .map(
        |(allowed_time,record_distance)| Race {
        allowed_time,
        record_distance
    }).map(|d| d.winning_distances().len()).fold(1, |a,b| a*b );

    print!("{}", result);
}

struct Race {
    allowed_time: u64,
    record_distance: u64
}

impl Race {
    fn winning_distances(&self) -> Vec<u64> {
        (0 .. self.allowed_time).map(| btn_time | {
            (self.allowed_time - btn_time) * btn_time
        }).filter(|&distance| distance > self.record_distance).collect()
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