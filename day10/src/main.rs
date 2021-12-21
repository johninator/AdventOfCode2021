use std::any::Any;

mod reader;

fn main() {

    let input_vecs = reader::read("input.txt").unwrap();
    let mut stack : Vec<Bracket> = Vec::new();
    let mut sum = 0;

    for input_vec in input_vecs {
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
                        println!("Expected ) but found instead");
                        sum += 3;
                        break;
                    }
                } else if ch == ']' {
                    if !matches!(stack.pop().unwrap(),Bracket::EDGY) {
                        println!("Expected ] but found instead");
                        sum += 57;
                        break;
                    }
                } else if ch == '}' {
                    if !matches!(stack.pop().unwrap(),Bracket::BRACES) {
                        println!("Expected }} but found instead");
                        sum += 1197;
                        break;
                    }
                } else { // ch == '>'
                    if !matches!(stack.pop().unwrap(),Bracket::TRIANGLE) {
                        println!("Expected > but found  instead");
                        sum += 25137;
                        break;
                    }
                }

            }
        }
    }

    println!("sum {}", sum);
}

enum Bracket
{
    ROUND,
    EDGY,
    BRACES,
    TRIANGLE
}
