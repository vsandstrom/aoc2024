use std::fs;

fn main() {
  let data = fs::read_to_string("day02/input")
    .expect("file parsing")
    .trim()
    .lines()
    .map(|s| 
      s.split(" ")
        .map(|n| 
          n.parse::<i32>()
          .unwrap()
        ).collect::<Vec<i32>>()
    ).collect::<Vec<Vec<i32>>>();

  let mut c = 0;
  for row in &data {
    if let Some(d) = dir(row[0], row[1]) {
      if safe(row, d) { c+=1; continue}
    }
  }
  println!("{}", c);

  let mut c = 0;
  for row in data {
    if let Some(d) = dir(row[0], row[1]) {
      if safe(&row, d) { c+=1; continue}
    }

    for i in 0..row.len() {
      let row = &[&row[..i], &row[(i+1)..]].concat();
      if let Some(d) = dir(row[0], row[1]) {
        if safe(row, d) { c+=1; break}
      }
    }
  }
  println!("{}", c);
}
  
fn dir(x: i32, y: i32) -> Option<bool> {
  match (x, y) {
    (x, y) if x < y => Some(true),
    (x, y) if x > y => Some(false),
    (x, y) if x == y => None,
    _ => panic!()
  }
}

pub fn safe(row: &[i32], dir: bool) -> bool { 
  for w in row.windows(2) {
    let tmp = {
      if dir { w[1] - w[0] } 
      else   { w[0] - w[1] }
    };
    if !(1..=3).contains(&tmp) {
      return false
    }
  }
  true
}
