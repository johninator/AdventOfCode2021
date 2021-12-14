use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;


// function takes path in the form of string slice and returns enum
// which contains vector of integers on success or IO error type, see `std::io::Error`
pub fn read(path: &str) -> Result<Vec<[i64; 4]>, io::Error> {
    let file = File::open(path)?; // open file by given path
                                  // wrap file into generic buffered reader, it will use 4 KB buffer internally
                                  // to reduce number of syscalls, thus improving performance
    let br = BufReader::new(file);
    // create an empty vector, type of the stored elements will be inferred
    let mut points_all = Vec::new();
    // br.lines() creates an iterator over lines in the reader
    // see: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines
    for line in br.lines() {
        // IO operations generally can return error, we check if got
        // an error,in which case we return this error as the function result
        let line = line?;
        let line_split : Vec<&str> = line.split("->").collect();
        let point_1 : Vec<i64> = line_split[0].split(",")
            .map(|s| s.trim())
            .map(|x| x.parse().unwrap()).collect();
        let point_2 : Vec<i64> = line_split[1].split(",")
            .map(|s| s.trim())
            .map(|x| x.parse().unwrap()).collect();
        
        let mut points : [i64; 4] = [0; 4];
        points[0] = point_1[0];
        points[1] = point_1[1];
        points[2] = point_2[0];
        points[3] = point_2[1];

        points_all.push(points);
    }
    Ok(points_all) // everything is Ok, return vector
}