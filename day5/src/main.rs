mod reader;
use reader::read;
use std::cmp;

fn main() {
    let points_all = read("input.txt").unwrap();

    let mut x_max: i64 = 0;
    let mut y_max: i64 = 0;

    for points in points_all.iter().copied() {
        x_max = cmp::max(x_max, points[0] + 1);
        x_max = cmp::max(x_max, points[2] + 1);
        y_max = cmp::max(y_max, points[1] + 1);
        y_max = cmp::max(y_max, points[3] + 1);
    }

    println!("x_max {}, y_max {}", x_max, y_max);

    let mut counters: Vec<i64> = vec![0; (x_max * y_max).try_into().unwrap()];

    for points in points_all {
        // vertical line
        if points[0] == points[2] {
            let x = points[0];
            let y_start = cmp::min(points[1], points[3]);
            let y_end = cmp::max(points[1], points[3]);
            for y in y_start..y_end + 1 {
                let index: usize = (y * x_max + x).try_into().unwrap();
                counters[index] = counters[index] + 1;
            }
        }
        // horizontal line
        else if points[1] == points[3] {
            let y = points[1];
            let x_start = cmp::min(points[0], points[2]);
            let x_end = cmp::max(points[0], points[2]);
            for x in x_start..x_end + 1 {
                let index: usize = (y * x_max + x).try_into().unwrap();
                counters[index] = counters[index] + 1;
            }
        }
        // diagonal line
        else {
            let x_start;
            let x_end ;
            let y_start;
            let y_incr;
            
            if points[0] < points[2] {
                x_start = points[0];
                x_end = points[2];
                y_start = points[1];
                if points[3] > points[1] {
                    y_incr = 1;
                } else {
                    y_incr = -1;
                }
            } else {
                x_start = points[2];
                x_end = points[0];
                y_start = points[3];
                if points[1] > points[3] {
                    y_incr = 1;
                } else {
                    y_incr = -1;
                }
            }

            let mut y = y_start;

            for x in x_start..x_end + 1 {
                let index: usize = (y * x_max + x).try_into().unwrap();
                counters[index] = counters[index] + 1;
                y = y + y_incr;
            }
        }
    }

    let result = counters.iter().filter(|&x| *x > 1).count();

    println!("result: {}", result);
}
