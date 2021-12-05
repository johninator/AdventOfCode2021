use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;

// function takes path in the form of string slice and returns enum
// which contains vector of integers on success or IO error type, see `std::io::Error`
pub fn read(path: &str) -> Result<Vec<[i64; 2]>, io::Error> {
    let file = File::open(path)?; // open file by given path
                                  // wrap file into generic buffered reader, it will use 4 KB buffer internally
                                  // to reduce number of syscalls, thus improving performance
    let br = BufReader::new(file);
    // create an empty vector, type of the stored elements will be inferred
    let mut v = Vec::new();
    // br.lines() creates an iterator over lines in the reader
    // see: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines
    for line in br.lines() {
        // IO operations generally can return error, we check if got
        // an error,in which case we return this error as the function result
        let line = line?;
        // let line_trimmed = line.trim(); // trim "whitespaces"
        let line_split = line.split(" ");
        let vec: Vec<&str> = line_split.collect();
        
        let val1 : i64;
        if vec[0] == "forward" {
            val1 = 0;
        } else if vec[0] == "up" {
            val1 = 1;
        } else {
            val1 = 2;
        }
        let val2 : i64 = vec[1].parse().unwrap();

        v.push([val1, val2]);
    }
    Ok(v) // everything is Ok, return vector
}