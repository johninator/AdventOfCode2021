use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub fn read(path: &str) -> Result<Vec<Vec<i64>>, io::Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);

    let mut map = Vec::new();

    for line in br.lines() {
        let line = line?;
        let integers : Vec<i64> = line.chars().map(|ch| i64::from(ch.to_digit(10).unwrap())).collect();
        map.push(integers);
    }
    Ok(map) 
}