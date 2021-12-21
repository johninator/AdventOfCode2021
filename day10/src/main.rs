use std::any::Any;

mod reader;

fn main() {

    let input_vecs = reader::read("input.txt").unwrap();
    let mut scores : Vec<i64> = Vec::new();

    for input_vec in input_vecs {

        let mut stack : Vec<Bracket> = Vec::new();
        let mut abort : bool = false;

        for ch in input_vec {

            if ch == '(' || ch == '[' || ch == '{' || ch == '<' {

                if ch == '(' {
                    stack.push(Bracket::ROUND);
                } else if ch == '[' {
                    stack.push(Bracket::EDGY);
                } else if ch == '{' {
                    stack.push(Bracket::BRACES);
                } else { // ch == '<'
                    stack.push(Bracket::TRIANGLE);
                }

            } else {
                if ch == ')' {
                    if !matches!(stack.pop().unwrap(),Bracket::ROUND) {
                        abort = true;
                        break;
                    }
                } else if ch == ']' {
                    if !matches!(stack.pop().unwrap(),Bracket::EDGY) {
                        abort = true;
                        break;
                    }
                } else if ch == '}' {
                    if !matches!(stack.pop().unwrap(),Bracket::BRACES) {
                        abort = true;
                        break;
                    }
                } else { // ch == '>'
                    if !matches!(stack.pop().unwrap(),Bracket::TRIANGLE) {
                        abort = true;
                        break;
                    }
                }
            }
        }

        if abort {
            continue;
        }

        let mut score = 0;
        for bracket in stack.iter().rev() {
            score *= 5;
            if matches!(bracket, Bracket::ROUND) {
                score += 1;
            } else if matches!(bracket, Bracket::EDGY) {
                score += 2;
            } else if matches!(bracket, Bracket::BRACES) {
                score += 3;
            } else {
                score += 4
            }
        }
        println!("score {}", score);
        scores.push(score);
    }

    scores.sort();
    println!("middle score {}",  scores[scores.len()/2]);
}

enum Bracket
{
    ROUND,
    EDGY,
    BRACES,
    TRIANGLE
}
