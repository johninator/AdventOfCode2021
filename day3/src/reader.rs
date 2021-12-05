use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

const SIZE : usize = 12;

// function takes path in the form of string slice and returns enum
// which contains vector of integers on success or IO error type, see `std::io::Error`
pub fn read(path: &str) -> Result<Vec<[i64; SIZE]>, io::Error> {
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
        
        let mut binary_code : [i64; SIZE] = [0;SIZE];

        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                binary_code[i] = 1;
            } else {
                binary_code[i] = -1;
            }
        }

        v.push(binary_code);
    }
    Ok(v) // everything is Ok, return vector
}