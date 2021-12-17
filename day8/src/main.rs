mod reader;
use reader::read;

fn main() {

    let entries = read("input.txt").unwrap();


    let counters : Vec<_> = entries.iter()
        .map(|x| x[10..14].iter().filter(|&e| (e.len() < 5 || e.len() == 7)).count()).collect();
    let result : usize = counters.iter().sum();

    println!("{:?}", counters);
    println!("{}", result);
}
