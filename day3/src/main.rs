mod reader;
use reader::read;

const SIZE : usize = 12;

fn main() {
    let reading_result = read("input.txt");

    let binary_codes = reading_result.unwrap();

    part2(&binary_codes);
}

fn part1(binary_codes: &Vec<[i64; SIZE]>) {
    let mut count_bits: [i64; SIZE] = [0; SIZE];

    for code in binary_codes {
        for (i, e) in code.iter().enumerate() {
            count_bits[i] = count_bits[i] + e;
        }
    }

    let mut gamma_rate: i64 = 0;
    let mut epsilon_rate: i64 = 0;
    let base: i64 = 2;

    for (i, e) in count_bits.iter().rev().enumerate() {
        if e > &0 {
            // '1' is the dominant bit -> gamme_rate
            gamma_rate = gamma_rate + base.pow(i.try_into().unwrap());
        } else {
            // '0' is the dominant bit -> epsilon_rate
            epsilon_rate = epsilon_rate + base.pow(i.try_into().unwrap());
        }
    }

    println!(
        "gamma-rate: {}, epsilon-rate: {}, mul: {}",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    );
}

fn part2(binary_codes: &Vec<[i64; SIZE]>) {
    let value_oxygen = compute_value_from_binary_codes(&binary_codes, true);
    let value_co2 = compute_value_from_binary_codes(&binary_codes, false);

    println!(
        "oxygen: {}, co2: {}, mul: {}",
        value_oxygen,
        value_co2,
        value_oxygen * value_co2
    );
}

fn compute_value_from_binary_codes(binary_codes: &Vec<[i64; SIZE]>, oxygen: bool) -> i64 {
    let mut value: i64 = 0;

    let mut count_bits: [i64; SIZE] = [0; SIZE];

    for code in binary_codes {
        for (i, e) in code.iter().enumerate() {
            count_bits[i] = count_bits[i] + e;
        }
    }

    let mut binary_codes: Vec<[i64; SIZE]> = binary_codes.clone();

    for i in 0..SIZE {
        let value_compare: i64;

        if count_bits[i] >= 0 {
            // '1' is the dominant bit or equal
            if oxygen {
                value_compare = 1;
            } else {
                value_compare = -1;
            }
        } else {
            // '0' is the dominant bit
            if oxygen {
                value_compare = -1;
            } else {
                value_compare = 1;
            }
        }

        let mut binary_codes_tmp: Vec<[i64; SIZE]> = Vec::new();

        for element in binary_codes {
            if element[i] == value_compare {
                binary_codes_tmp.push(element);
            }
        }
        binary_codes = binary_codes_tmp.clone();

        if binary_codes.len() == 1 {
            value = compute_value_from_binary_code(binary_codes.first().unwrap());
        } else {
            // recompute bit counter
            count_bits = [0; SIZE];
            for code in &binary_codes {
                for (i, e) in code.iter().enumerate() {
                    count_bits[i] = count_bits[i] + e;
                }
            }
        }
    }

    return value;
}

fn compute_value_from_binary_code(binary_code: &[i64; SIZE]) -> i64 {
    let mut value: i64 = 0;
    let base: i64 = 2;

    for (i, e) in binary_code.iter().rev().enumerate() {
        if e == &1 {
            value = value + base.pow(i.try_into().unwrap());
        }
    }
    return value;
}
