mod reader;
use reader::read;

fn main() {

    let entries = read("input.txt").unwrap();

    println!("{:?}", entries);
}
