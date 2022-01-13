use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub fn read(path: &str) -> Result<Vec<Vec<i32>>, io::Error> {

    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut cave = Vec::new();

    for line in br.lines() {
        let line = line?;
        let line_int : Vec<i32> = line.chars().map(|x| x.to_digit(10).unwrap() as i32).collect();
        cave.push(line_int);
    }
    Ok(cave)
}