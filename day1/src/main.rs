use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;

// function takes path in the form of string slice and returns enum
// which contains vector of integers on success or IO error type, see `std::io::Error`
fn read(path: &str) -> Result<Vec<i64>, io::Error> {
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
        let n = line
            .trim() // trim "whitespaces"
            .parse() // call `str::parse::<i64>(&self)` method on the trimmed line, which parses integer
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?; // parse() can return error (e.g. for string "abc"), here if we got it, we convert it to `std::io::Error` type and return it as function result
        v.push(n); // push acquired integer to the vector
    }
    Ok(v) // everything is Ok, return vector
}

fn compute_sum(pos: usize, heights: &Vec<i64>) -> i64 {
    let sum = heights[pos] + heights[pos-1] + heights[pos-2];
    println!("sum is {}", sum);
    return sum;
}

fn main() {
    let reading_result = read("input.txt");
    // println!("{:?}", heights);

    let heights_good;

    match reading_result {
        Ok(v) => heights_good = v,
        Err(e) => return,
    }

    let mut count_increased = 0;
    let mut sum_previous = 0;
    let mut sum = 0;

    for (pos, e) in heights_good.iter().enumerate() {
        // iterate immutably

        if pos > 1 {

            sum = compute_sum(pos, &heights_good);
        }

        if pos > 2 {
            if sum > sum_previous {
                count_increased = count_increased + 1;
            }
        }

        sum_previous = sum;
    }

    println!("Increased {} times", count_increased);
}
