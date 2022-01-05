use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub fn read(path: &str) -> Result<Vec<[String;2]>, io::Error> {

    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut edges = Vec::new();

    for line in br.lines() {
        let line = line?;
        let line_split : Vec<&str> = line.split('-').map(|x| x.trim()).collect();
        let mut edge : [String; 2] = [String::new(), String::new()];
        edge[0] = line_split[0].to_string();
        edge[1] = line_split[1].to_string();
        edges.push(edge);
    }
    Ok(edges)
}