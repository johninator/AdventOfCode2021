use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub fn read(path: &str) -> Result<Vec<[usize;2]>, io::Error> {

    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut points = Vec::new();

    for line in br.lines() {
        let line = line?;
        let line_split : Vec<usize> = line.split(',').map(|x| x.trim().parse().unwrap()).collect();
        let mut point : [usize; 2] = [0; 2];
        point[0] = line_split[0];
        point[1] = line_split[1];
        points.push(point);
    }
    Ok(points)
}