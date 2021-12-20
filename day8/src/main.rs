mod reader;
use reader::read;
use std::collections::HashMap;


fn main() {
    let entries = read("input.txt").unwrap();

    let mut sum : i64 = 0;

    for entry in entries {
        let signals : Vec<_> = entry[0..10].to_vec();
        let digits : Vec<_> = entry[10..14].to_vec();

        let digits = deduce_signals(&signals, &digits);
        sum += digits[0] * 1000 + digits[1] * 100 + digits[2] * 10 + digits[3];
    }
    println!("sum: {}", sum);
}

fn deduce_signals(signals : &Vec<String>, digits : &Vec<String>) -> [i64; 4]
{
    let mut mapping  = HashMap::new();

    let mut signals_sorted = signals.clone();
    signals_sorted.sort_by(|a,b| a.len().cmp(&b.len()));

    let mut cf = String::new();
    let mut bd = String::new();
    let mut a = String::new();
    let mut g = String::new();

    for i in 0..5 {

    for signal in signals_sorted.clone()
    {
        if signal.len() == 2 && i == 0 { // 1
            cf = signal;
            break;
        } else if signal.len() == 3 && i == 1 {
            // resolve 'a'
            for ch in signal.chars() {
                if !cf.contains(ch) {
                    a.push(ch);
                }
            }
            mapping.insert("a", a.chars().nth(0).unwrap());
            break;
        } else if signal.len() == 4  && i == 2 { // 7
            for ch in signal.chars() {
                if !cf.contains(ch) {
                    bd.push(ch);
                }
            }
            break;
        } else if signal.len() == 5 && i == 3 && // 3
                  signal.contains(cf.chars().nth(0).unwrap()) && 
                  signal.contains(cf.chars().nth(1).unwrap())  {
          // resolve 'b' & 'd'
          let bd_0 = bd.chars().nth(0).unwrap();
          let bd_1 = bd.chars().nth(1).unwrap();
          if signal.contains(bd_0) {
            mapping.insert("d", bd_0);
            mapping.insert("b", bd_1);
          } else {
            mapping.insert("d", bd_1);
            mapping.insert("b", bd_0);
          }
          break;
        } else if signal.len() == 5 && i == 4 && // 5
                 signal.contains(*mapping.get("b").unwrap())  {
            // resolve 'c' & 'f'
            let cf_0 = cf.chars().nth(0).unwrap();
            let cf_1 = cf.chars().nth(1).unwrap();
            if signal.contains(cf_0) {
              mapping.insert("f", cf_0);
              mapping.insert("c", cf_1);
            } else {
              mapping.insert("f", cf_1);
              mapping.insert("c", cf_0);
            }
            // resolve 'g'
            for ch in signal.chars() {
                if !cf.contains(ch) &&
                !bd.contains(ch) &&
                !a.contains(ch) {
                    g.push(ch);
                }
            }
            mapping.insert("g", g.chars().nth(0).unwrap());
            // resolve 'e'
            let mut all = String::from("abcdefg");
            for (key, val) in mapping.clone() {
                if all.contains(val) {
                  all = all.replace(val, "");
                }
            }
            mapping.insert("e", all.chars().nth(0).unwrap());
            break;
        }
   }
}

   let mut digits_result : [i64;4] = [0;4];

   for (i,digit) in digits.iter().enumerate() {
       let mut digit_resolved = String::new();
       for char_digit in digit.chars() {
        // resolve digit with mapping
        for (key, val) in mapping.clone() {
            if val == char_digit {
                digit_resolved.push(key.chars().nth(0).unwrap());
            }
            
        }
       }

       let mut digit_resolved_vec = digit_resolved.chars().collect::<Vec<char>>();
       digit_resolved_vec.sort_unstable();
       let digit_resolved_sorted : String = digit_resolved_vec.into_iter().collect();

    if digit_resolved_sorted == "abcefd" {
        digits_result[i] = 0;
    } else if digit_resolved_sorted == "cf" {
         digits_result[i] = 1;
    } else if digit_resolved_sorted == "acdeg" {
            digits_result[i] = 2;
    } else if digit_resolved_sorted == "acdfg" {
                digits_result[i] = 3;
    } else if digit_resolved_sorted == "bcdf" {
         digits_result[i] = 4;
    } else if digit_resolved_sorted == "abdfg" {
        digits_result[i] = 5;
    } else if digit_resolved_sorted == "abdefg" {
        digits_result[i] = 6;
    } else if digit_resolved_sorted == "acf" {
        digits_result[i] = 7;
    } else if digit_resolved_sorted == "abcdefg" {
        digits_result[i] = 8;
    } else if digit_resolved_sorted == "abcdfg" {
        digits_result[i] = 9;
    }
   }

   return digits_result;
}
