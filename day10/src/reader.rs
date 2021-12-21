use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub fn read(path: &str) -> Result<Vec<Vec<char>>, io::Error> {

    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut brackets = Vec::new();

    for line in br.lines() {
        let line = line?;
        let line_chars : Vec<char> = line.chars().collect();
        brackets.push(line_chars);
    }
    Ok(brackets)
}