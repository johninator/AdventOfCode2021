mod reader;
use reader::read;
use std::cmp;

fn main() {
    let map = reader::read("input.txt").unwrap();
    let local_minima = find_local_minima(&map);

    let risk_levels : Vec<i64> = local_minima.iter().map(|x| x[2] + 1).collect();
    let sum : i64 = risk_levels.iter().sum();

    println!("sum {}", sum);
    
}

fn find_local_minima(map: &Vec<Vec<i64>>) -> Vec<[i64; 3]> {
    let rows = map.len();
    let cols = map[0].len();
    let mut local_minima: Vec<[i64; 3]> = Vec::new();

    for row in 0..rows {
        for col in 0..cols {
            let row_local_start: usize;
            let col_local_start: usize;

            if row == 0 {
                row_local_start = 0;
            } else {
                row_local_start = cmp::max(0, row - 1);
            }

            if col == 0 {
                col_local_start = 0;
            } else {
                col_local_start = cmp::max(0, col - 1);
            }

            let row_local_end = cmp::min(rows, row + 2);
            let col_local_end = cmp::min(cols, col + 2);

            let mut is_local_minimum : bool = true;
            let val = map[row][col];

            for row_local in row_local_start..row_local_end {
                for col_local in col_local_start..col_local_end {
                    let val_local = map[row_local][col_local];
                    if val_local <= val &&
                       (row_local != row || col_local != col) {
                        is_local_minimum = false;
                    }
                }
            }

            if is_local_minimum {
                local_minima.push([row.try_into().unwrap(), col.try_into().unwrap(), val]);
            }
        }
    }
    return local_minima;
}
