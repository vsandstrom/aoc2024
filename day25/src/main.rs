use std::borrow::{Borrow, BorrowMut};

fn main() {
  let mut argv = std::env::args();
  let mut data = std::fs::read_to_string(
    format!("day25/{}", argv.borrow_mut().nth(1).clone().unwrap_or("test".to_string())))
    .expect("no input file");

  let mut keys = vec!();
  let mut locks = vec!();
  for keylock in data.split("\n\n") {
    let c = keylock.chars().collect::<Vec<char>>();
    match c[..5] {
      ['#', '#', '#', '#', '#'] => {
        keys.push(parse_key(&c));

      }
      ['.', '.', '.', '.', '.'] => {
        locks.push(parse_lock(&c));
      },
      _ => ()
    }
  }


  let mut sum = 0;

  for lock in &locks {
    for key in &keys {
      if lock[0] > key[0] { continue; }
      if lock[1] > key[1] { continue; }
      if lock[2] > key[2] { continue; }
      if lock[3] > key[3] { continue; }
      if lock[4] > key[4] { continue; }
      sum+=1;
    }
  }
  // println!("{:?}", keys);
  // println!("{:?}", locks);
  println!("{}", sum);
}

fn parse_lock(lock: &[char]) -> [u32; 5] {
  let lock:String = lock.iter().collect();
  let mut v = [0; 5];
  for row in lock.lines().skip(1).take(5) {
    let c: Vec<_> = row.chars().collect();
    v[0] += if c[0] =='#' {1} else {0};
    v[1] += if c[1] =='#' {1} else {0};
    v[2] += if c[2] =='#' {1} else {0};
    v[3] += if c[3] =='#' {1} else {0};
    v[4] += if c[4] =='#' {1} else {0};
  }
  v
}

fn parse_key(key: &[char]) -> [u32; 5] {
  let key:String = key.iter().collect();
  let mut v = [0; 5];
  for row in key.lines().skip(1).take(5) {
    let c: Vec<_> = row.chars().collect();
    v[0] += if c[0] =='#' {1} else {0};
    v[1] += if c[1] =='#' {1} else {0};
    v[2] += if c[2] =='#' {1} else {0};
    v[3] += if c[3] =='#' {1} else {0};
    v[4] += if c[4] =='#' {1} else {0};
  }
  for x in v.iter_mut() {
    *x = 5-*x; 
  }
  v
}
