mod reader;
use reader::read;


fn main() {

    let reading_result = read("input.txt");

    let binary_codes = reading_result.unwrap();

    part1(&binary_codes);    
}


fn part1(binary_codes : &Vec<[i64; 12]>)
{
   let mut count_bits : [i64; 12] = [0,0,0,0,0,0,0,0,0,0,0,0];

   for code in binary_codes {
       for (i, e) in code.iter().enumerate() {
        count_bits[i] = count_bits[i] + e;
       }
   }

   let mut gamma_rate : i64 = 0;
   let mut epsilon_rate : i64 = 0;
   let base: i64 = 2;

   for (i, e) in count_bits.iter().rev().enumerate() {
        if e > &0 { // '1' is the dominant bit -> gamme_rate
            gamma_rate = gamma_rate + base.pow(i.try_into().unwrap());
        } else {  // '0' is the dominant bit -> epsilon_rate
            epsilon_rate = epsilon_rate + base.pow(i.try_into().unwrap());
        }
   }


   println!("gamma-rate: {}, epsilon-rate: {}, mul: {}",
   gamma_rate, epsilon_rate, gamma_rate *epsilon_rate);
}

