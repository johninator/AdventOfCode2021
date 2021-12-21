mod reader;

use std::cmp;

fn main() {
    let mut energies = reader::read("input.txt").unwrap();
    let mut flashes = 0;
    let steps = 1000;

    for step in 0..steps {
        increase_energy(&mut energies);
        if !flash(&mut energies, &mut flashes) {
            println!("step: {}", step+1);
            return;
        }
    }

    println!("{}", flashes);
}


fn increase_energy(energies: &mut Vec<u32>)
{
    *energies = energies.iter().map(|x| x + 1).collect();
}


fn flash(energies: &mut Vec<u32>, flashes: &mut i64) -> bool
{
    let mut flashes_left = energies.iter().filter(|&&x| x == 10).count();

    let mut counter_flashes = 0;

    while flashes_left > 0
    {
        let mut index : usize = 0;

        for (index_search, energy_search) in energies.iter().enumerate()
        {
            if *energy_search == 10 {
                index = index_search;
                break;
            }
        }

        flash_single(energies, index);
        *flashes += 1;
        counter_flashes += 1;
        flashes_left = energies.iter().filter(|&&x| x == 10).count();
    }

    *energies = energies.iter().map(|&x| {
        if x > 9 { 0 } else { x }
    }).collect();

    if counter_flashes == 100 {
        return false;
    }

    return true;
}

fn flash_single(energies: &mut Vec<u32>, index_flat: usize)
{
    let rows = 10;
    let cols = 10;
    let row = index_flat / rows;
    let col = index_flat % cols;

    
    let row_start;
    let col_start;

    if row == 0 {
        row_start = 0;
    } else {
        row_start = cmp::max(0, row - 1);
    }

    if col == 0 {
        col_start = 0;
    } else {
        col_start = cmp::max(0, col - 1);
    }

    let row_end = cmp::min(rows, row + 2);
    let col_end = cmp::min(cols, col + 2);

    energies[row * cols + col] = 11;

    for row_local in row_start..row_end {
        for col_local in col_start..col_end {
            let index = row_local * cols + col_local;
            if energies[index] < 10 {
                energies[index] += 1;
            }
        }
    }
}
