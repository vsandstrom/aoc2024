use std::fs;

fn main() {
  let data = fs::read_to_string("day4/input").expect("file parsing").lines().map(|s| s.to_string().chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

  let mut c = 0;
  for row in &data {
    for win in row.windows(4) {
      match win {
        ['X','M','A','S'] | ['S','A','M','X'] => c+=1,
        _ => ()
      }
    }
  }

  let l = data[0].len();
  for win in data.windows(4) {
    for i in 0..l {
      if i < l-3 {
        match &[win[0][i], win[1][i+1], win[2][i+2], win[3][i+3]] {
          ['X','M','A','S'] | ['S','A','M','X'] => c+=1,
          _ => ()
        }
        
        match &[win[0][i+3], win[1][i+2], win[2][i+1], win[3][i]] {
          ['X','M','A','S']| ['S','A','M','X'] => c+=1,
          _ => ()
        }
      }
      match &[win[0][i],win[1][i],win[2][i],win[3][i]] {
        ['X','M','A','S'] | ['S','A','M','X'] => c+=1,
        _ => ()
      }
    }
  }

  println!("{}", c);

// ========================================================

  let mut c = 0;
  let l = data[0].len();
  for win in data.windows(3) {
    for i in 0..l-2 {
      match &[win[0][i], win[0][i+2], win[1][i+1], win[2][i], win[2][i+2]] {
        ['M','M','A','S','S'] | ['S','M','A','S','M'] | ['S','S','A','M','M'] | ['M','S','A','M','S'] => c+=1,
        _ => ()
      } 
    }
  }
  println!("{}", c);
}
