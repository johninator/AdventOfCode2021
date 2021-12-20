mod reader;
use std::cmp;

fn main() {
    let map = reader::read("input.txt").unwrap();
    let local_minima = find_local_minima(&map);
    let mut map_binary = create_binary_map(&map);

    let mut size_basins= Vec::new();
    for local_minimum in local_minima {
        size_basins.push(find_basin(&mut map_binary, [local_minimum[0],local_minimum[1]]));
    }

    size_basins.sort_by(|a,b| b.cmp(a));
    println!("size_basins {}", size_basins[0] * size_basins[1] * size_basins[2]);
}

fn find_local_minima(map: &Vec<Vec<i64>>) -> Vec<[i64; 3]>
{
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

fn create_binary_map(map: &Vec<Vec<i64>>) -> Vec<Vec<i64>>
{
    let binary_map : Vec<Vec<i64>> = map
        .iter()
        .map(|row| row.iter().map(|&x|
         {
             if x >= 9 { 1 }
             else { 0 }
         })
        .collect()).collect();
    return binary_map;
}

fn find_basin(map_binary: &mut Vec<Vec<i64>>, local_minimum: [i64;2]) -> i64
{
    let mut position = local_minimum;
    let mut positions_next = Vec::new();
    let mut size_basin = 1;
    let mut row : usize = local_minimum[0].try_into().unwrap();
    let mut col : usize = local_minimum[1].try_into().unwrap();
    map_binary[row][col] = 1;

    find_next_positions(map_binary, &mut positions_next, position);

    while !positions_next.is_empty() {

        position = positions_next.pop().unwrap();
        row = position[0].try_into().unwrap();
        col = position[1].try_into().unwrap();

        if map_binary[row][col] == 0 {
            map_binary[row][col] = 1;
            size_basin += 1;
        }
        find_next_positions(map_binary, &mut positions_next, position);
    }
    return size_basin;
}

fn find_next_positions(map_binary: &Vec<Vec<i64>>,
                 positions_next : &mut Vec<[i64; 2]>,
                 local_minimum: [i64;2])
{
    let rows = map_binary.len();
    let cols = map_binary[0].len();

    // decision order: left -> up -> right -> down
    let row : usize = local_minimum[0].try_into().unwrap();
    let col : usize = local_minimum[1].try_into().unwrap();

    // left
    if col != 0 && map_binary[row][col-1] == 0
    {
        let col_new = col - 1;
        positions_next.push([row.try_into().unwrap(),
            col_new.try_into().unwrap()]);
    }
    // up
    if row != 0 && map_binary[row-1][col] == 0
    {
        let row_new = row - 1;
        positions_next.push([row_new.try_into().unwrap(),
            col.try_into().unwrap()]);
    }
    // right
    if col != cols-1 && map_binary[row][col+1] == 0
    {
        let col_new = col + 1;
        positions_next.push([row.try_into().unwrap(),
            col_new.try_into().unwrap()]);
    }
    // down
    if row != rows-1 && map_binary[row+1][col] == 0
    {
        let row_new = row + 1;
        positions_next.push([row_new.try_into().unwrap(),
            col.try_into().unwrap()]);
    }
}
