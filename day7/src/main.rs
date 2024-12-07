use std::fs;
mod task1;
mod task2;

fn main() {
  
  // check if value on other branch on current recursive depth is equal to an other.
  // let hm: HashMap<usize, i32> = HashMap::new();
  let data = fs::read_to_string("day7/input").expect("no input file")
    .lines()
    .map(|l| 
      l.split(&[':', ' '])
      .filter_map(|s| 
        s.parse::<u64>()
        .ok())
      .collect::<Vec<u64>>())
    .collect::<Vec<Vec<u64>>>();


  let mut  sum = 0;
  for row in &data {
    let val = row[0];
    let nums = &row[1..];
    if task1::mul(val, nums) || task1::add(val, nums) {
      sum += val;
    }
  }
  println!("task1: {}", sum);


  sum = 0;
  for row in data {
    let val = row[0];
    let nums = &row[1..];
    if task2::mul(val, nums) || task2::add(val, nums) || task2::cat(val, nums) {
      sum += val;
    }
  }
  println!("task2: {}", sum);

}


