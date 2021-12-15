mod reader;
use reader::read;

fn main() {
    
    let reading_result = read("input.txt").unwrap();

    let input_random = reading_result.0;
    let boards = reading_result.1;

    let mut counters : Vec<[i64; 25]> = vec![[0;25]; boards.len()];

    let mut counter_wins : Vec<i64> = vec![0; boards.len()]; 

    for number in input_random {
        for (i_board, board) in boards.iter().enumerate() {
            for i in 0..25 {
                if board[i] == number {
                    counters[i_board][i] = counters[i_board][i] + 1;

                    if counter_wins[i_board] == 0 &&
                       check_board(counters[i_board]) == true {
                        let sum = compute_sum_board(board, counters[i_board]);
                        let result = sum * number;
                        println!("finished! Board {}, Sum {}, Number {}, Result: {}",
                                 i_board, sum, number, result);
                        counter_wins[i_board] = 1 ;
                    }
                }
            }
        }
    }
}

fn check_board(board_counter : [i64; 25]) -> bool 
{
  // vertical lines
  for x in 0..5 {
    let mut counter_ones = 0;
    for y in 0..5 {
      let index = y*5 + x;
      if board_counter[index] ==  1 {
        counter_ones = counter_ones + 1;
      }
    }
    if counter_ones == 5 {
        return true;
    }
  }
  // horizontal lines
  for y in 0..5 {
    let mut counter_ones = 0;
    for x in 0..5 {
      let index = y*5 + x;
      if board_counter[index] == 1 {
        counter_ones = counter_ones + 1;
      }
    }
    if counter_ones == 5 {
        return true;
    }
  }
  return false;
}

fn compute_sum_board(board : &Vec<i64>, board_counter : [i64; 25]) -> i64 
{
  let mut sum : i64 = 0;

  for y in 0..5 {
    for x in 0..5 {
      let index = y*5 + x;
      if board_counter[index] == 0 {
        sum = sum + board[index];
      }
    }
  }
  return sum;
}
