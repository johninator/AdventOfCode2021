mod reader;
use reader::read;


fn main() {

    let reading_result = read("input.txt");

    let commands = reading_result.unwrap();

    part1(&commands);    
    part2(&commands);    
}


fn part1(commands : &Vec<[i64; 2]>)
{
    let mut pos_hor : i64 = 0;
    let mut pos_depth : i64 = 0;

    for command in commands {
        if command[0] == 0 { // forward
            pos_hor = pos_hor + command[1];
        } else if command[0] == 1 { // up
            pos_depth = pos_depth - command[1];
        }  else { // down
            pos_depth = pos_depth + command[1];
        }
    }

    println!("part 1: hor: {}, depth: {}, mul {}",
              pos_hor, pos_depth, pos_hor * pos_depth);
}

fn part2(commands : &Vec<[i64; 2]>)
{
    let mut pos_hor : i64 = 0;
    let mut pos_depth : i64 = 0;
    let mut aim : i64 = 0;

    for command in commands {
        if command[0] == 0 { // forward
            pos_hor = pos_hor + command[1];
            pos_depth = pos_depth + command[1] * aim;
        } else if command[0] == 1 { // up
            aim = aim - command[1];
        }  else { // down
            aim = aim + command[1];
        }
    }

    println!("part 2: hor: {}, depth: {}, mul {}",
              pos_hor, pos_depth, pos_hor * pos_depth);
}