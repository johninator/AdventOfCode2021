mod reader;
use reader::read;

fn main() {
    let entries = read("input.txt").unwrap();

    let mut sum : i64 = 0;

    for entry in entries {
        let signals : Vec<_> = entry[0..10].to_vec();
        let digits : Vec<_> = entry[10..14].to_vec();

        let digits = deduce_signals(&signals, &digits);
        sum += digits.iter().sum::<i64>();
    }
    println!("sum: {}", sum);
}

fn deduce_signals(signals : &Vec<String>, digits : &Vec<String>) -> [i64; 4]
{
    let mut mapping : [i64; 10];
    let mut signals_sorted = signals.clone();
    signals_sorted.sort_by(|a,b| a.len().cmp(&b.len()));

    for signal in signals_sorted
    {
        if signal.len() == 2 {

        }



    }

    return [0;4];
}

fn map_length_to_chars_digit(length : usize) -> Vec<Vec<char>>
{
    let mut chars = Vec::new();

    if length == 2 {
        // 1
        chars.push(vec!['c','f']);
    } else if length == 3 {
        // 7
        chars.push(vec!['a','c','f']);
    } else if length == 4 {
        // 4
        chars.push( vec!['b','c','d','f']);
    } else if length == 7 {
        // 8
        chars.push( vec!['a','b','c','d','e','f','g']);
    } else if length == 5 {
        // 2,3,5
        chars.push( vec!['a','c','d','e','g']);
        chars.push( vec!['a','c','d','f','g']);
        chars.push( vec!['a','b','d','f','g']);
    } else {
        // 9,0,6
        chars.push( vec!['a','b','c','d','f','g']);
        chars.push( vec!['a','b','c','e','f','g']);
        chars.push( vec!['a','b','d','e','f','g']);
    }
    return chars;
}
