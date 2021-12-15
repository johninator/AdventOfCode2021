use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;


pub fn read(path: &str) -> Result<(Vec<i64>, Vec<Vec<i64>>), io::Error> {
    let file = File::open(path)?; // open file by given path
                                  // wrap file into generic buffered reader, it will use 4 KB buffer internally
                                  // to reduce number of syscalls, thus improving performance
    let br = BufReader::new(file);

    let mut boards : Vec<Vec<i64>> = Vec::new();
    let mut line_integers : Vec<i64>;

    let lines : Vec<_> = br.lines().map(|x| x.unwrap()).collect();
    let num_lines = lines.len();

    let line_1st = lines.get(0).unwrap();
    let line_1st_split : Vec<&str> = line_1st.split(",").collect();
    let input_random : Vec<i64> = line_1st_split.iter().map(|x| x.parse().unwrap()).collect();

    let mut board : Vec<i64> = Vec::new();

    for i in 2..num_lines {

        let line = lines.get(i).unwrap();

        let line_split : Vec<&str> = line.split_whitespace().collect();
        line_integers  = line_split.iter().map(|x| x.parse().unwrap()).collect();
        board.append(&mut line_integers);

        if board.len() == 25 {
            boards.push(board.clone());  
            board.clear();

            continue;
        }
    }

    Ok((input_random, boards))
}