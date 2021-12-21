use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub fn read(path: &str) -> Result<Vec<u32>, io::Error> {

    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut energies = Vec::new();

    for line in br.lines() {
        let line = line?;
        let mut line_ints : Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        energies.append(&mut line_ints);
    }
    Ok(energies)
}