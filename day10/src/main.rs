use std::{
  collections::{HashSet, VecDeque}, 
  fs,
  env::args,
};

fn main() {
  let mut argv = args();
  let file = fs::read_to_string(format!("day10/{}", argv.nth(1).unwrap_or("test".to_string())))
    .expect("no input file")
    .lines()
    .map(|l| l.bytes().map(|b| {b-b'0'}).collect::<Vec<u8>>())
    .collect::<Vec<Vec<u8>>>();

  let mut start_pos = vec!();

  for (i, row) in file.iter().enumerate() {
    for (j, c) in row.iter().enumerate() {
      if *c == 0 { start_pos.push((i, j)); }
    }
  }

  printmap(&file);

  let h = file.len();
  let w = file[0].len();

  // hashmap to keep track of start_pos to summit trails
  let mut frontier = vec!();
  let mut peaks = vec![0; start_pos.len()];

  // * PART 1 *

  println!("{:?}", start_pos);
  for (i, spos) in start_pos.iter().enumerate() {
    let mut travelled: HashSet<(usize, usize)> = HashSet::new();
    frontier.push(*spos);
    println!("{:?}", frontier);
    while let Some(pos) = frontier.pop() {
      println!("{:?}", frontier);
      let val = file[pos.0][pos.1];
      if val == 9 {
        peaks[i]+=1;
      }

      if pos.1 != 0 {
        let p = (pos.0, pos.1-1);
        if let Some(c) = file[p.0].get(p.1) {
          validate_and_expand(val, *c, &p, &travelled, &mut frontier);
        } 
      }

      if pos.1 != w-1 {
        let p = (pos.0, pos.1+1);
        if let Some(c) = file[pos.0].get(pos.1+1) {
          validate_and_expand(val, *c, &p, &travelled, &mut frontier);
        }
      }

      if pos.0 != 0 {
        let p = (pos.0-1, pos.1);
        if let Some(row) = file.get(p.0) {
          let c = row[p.1];
          validate_and_expand(val, c, &p, &travelled, &mut frontier);
        } 
      }

      if pos.0 != h-1 {
        let p = (pos.0+1, pos.1);
        if let Some(row) = file.get(p.0) {
          let c = row[p.1];
          validate_and_expand(val, c, &p, &travelled, &mut frontier);
        }
      }
      travelled.insert(pos);
    }
  }

  println!("{}", peaks.iter().sum::<i32>());
  
  // * PART 1 *
  // hashmap to keep track of start_pos to summit trails
  let mut frontier = VecDeque::new();
  let mut peaks = vec![0; start_pos.len()];

  // * PART 1 *

  println!("{:?}", start_pos);
  for (i, spos) in start_pos.iter().enumerate() {
    let mut travelled: HashSet<(usize, usize)> = HashSet::new();
    frontier.push_front(*spos);
    println!("{:?}", frontier);
    while let Some(pos) = frontier.pop_back() {
      println!("{:?}", frontier);
      let val = file[pos.0][pos.1];
      if val == 9 {
        peaks[i]+=1;
      }

      if pos.1 != 0 {
        let p = (pos.0, pos.1-1);
        if let Some(c) = file[p.0].get(p.1) {
          validate_and_expand2(val, *c, &p, &travelled, &mut frontier);
        } 
      }

      if pos.1 != w-1 {
        let p = (pos.0, pos.1+1);
        if let Some(c) = file[pos.0].get(pos.1+1) {
          validate_and_expand2(val, *c, &p, &travelled, &mut frontier);
        }
      }

      if pos.0 != 0 {
        let p = (pos.0-1, pos.1);
        if let Some(row) = file.get(p.0) {
          let c = row[p.1];
          validate_and_expand2(val, c, &p, &travelled, &mut frontier);
        } 
      }

      if pos.0 != h-1 {
        let p = (pos.0+1, pos.1);
        if let Some(row) = file.get(p.0) {
          let c = row[p.1];
          validate_and_expand2(val, c, &p, &travelled, &mut frontier);
        }
      }
      travelled.insert(pos);
    }
  }
  println!("{}", peaks.iter().sum::<i32>());
}

fn validate_and_expand(a: u8, b:u8, pos: &(usize, usize), travelled: &HashSet<(usize, usize)>, frontier: &mut Vec<(usize, usize)>) {
  if a + 1 == b && !travelled.contains(pos) {
    frontier.push(*pos);
  }
}

fn validate_and_expand2(a: u8, b:u8, pos: &(usize, usize), travelled: &HashSet<(usize, usize)>, frontier: &mut VecDeque<(usize, usize)>) {
  if a + 1 == b && !travelled.contains(pos) {
    frontier.push_front(*pos);
  }
}

fn printmap(map: &[Vec<u8>]) {
  for row in map.iter() {
    for n in row {
      print!("{}", n);
    }
    println!();
  }
}
