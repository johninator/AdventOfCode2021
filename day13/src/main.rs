mod reader;

use std::collections::HashSet;

fn main() {

    let mut points = reader::read("input.txt").unwrap();
    // true == y, false == x
    let foldings: Vec<(bool, usize)> = vec![
        (false, 655),
        (true, 447),
        (false, 327),
        (true, 223),
        (false, 163),
        (true, 111),
        (false, 81),
        (true, 55),
        (false, 40),
        (true, 27),
        (true, 13),
        (true, 6)];

    for folding in foldings {
        fold(&mut points, folding);
    }
    print_paper(points.clone());

    let set_dots : HashSet<[usize;2]> = HashSet::from_iter(points.iter().cloned());
    println!("number of dots: {}", set_dots.len());
}

fn fold(points : &mut Vec<[usize; 2]>, folding : (bool, usize)) {

    let folding_along_y = folding.0;
    let folding_axis_value = folding.1;

    for point in points {
        if folding_along_y && point[1] > folding_axis_value {
            // vertical folding
            point[1] -= 2*(point[1] - folding_axis_value);
        } else if !folding_along_y && point[0] > folding_axis_value {
            // horizontal folding
            point[0] -= 2*(point[0] - folding_axis_value);
        }
    }
}

fn print_paper(points : Vec<[usize; 2]>) {
    let x_values : Vec<usize> = points.iter().map(|x| x[0]).collect();
    let y_values : Vec<usize> = points.iter().map(|x| x[1]).collect();

    let x_max : usize  = (*x_values.iter().max().unwrap()).try_into().unwrap();
    let y_max : usize  = (*y_values.iter().max().unwrap()).try_into().unwrap();

    let mut string_dots : Vec<usize> = vec![0; (x_max+1)*(y_max+1)];

    for point in points {
        let index : usize = (point[0] + point[1] * x_max).try_into().unwrap();
        string_dots[index] = 1;
    }

    println!();
    for row in 0..y_max+1 {
        let slice  =  string_dots[row*x_max..(row+1)*x_max].to_vec();
        println!("{:?}", slice);
    }
}

