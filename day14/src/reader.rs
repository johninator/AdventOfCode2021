use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub fn read(path: &str) -> Result<Vec<[i64;3]>, io::Error> {

    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut rules = Vec::new();

    for line in br.lines() {
        let line = line?;
        let line_split : Vec<&str> = line.split("->").map(|x| x.trim()).collect();

        let input_encoded_1 : u32 =  line_split[0].chars().nth(0).unwrap() as u32 - 'A' as u32;
        let input_encoded_2 : u32 =  line_split[0].chars().nth(1).unwrap() as u32 - 'A' as u32;
        let input_encoded = input_encoded_1 * 26 + input_encoded_2;

        let output_encoded : u32 =  line_split[1].chars().nth(0).unwrap() as u32 - 'A' as u32;
        let output_encoded_1 : u32 = 26 * input_encoded_1 + output_encoded;
        let output_encoded_2 : u32 = 26 * output_encoded + input_encoded_2;

        let mut rule : [i64; 3] = [0; 3];
        rule[0] = input_encoded.try_into().unwrap();
        rule[1] = output_encoded_1.try_into().unwrap();
        rule[2] = output_encoded_2.try_into().unwrap();
        rules.push(rule);
    }
    Ok(rules)
}