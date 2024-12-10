use std::{collections::{hash_map::Entry, HashMap, HashSet}, fs, vec};
use std::ops::Neg;

fn main() {
  let mut hm: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
  let antennas: Vec<Vec<char>> = fs::read_to_string("day8/input").expect("no input file")
    .lines()
    .map(|l| l.chars().collect()).collect();

  let h = antennas.len();
  let w = antennas[0].len();
      
  for (i, l) in antennas.iter().enumerate() {
    for (j, c) in l.iter().enumerate() {
      if *c != '.' { 
        match hm.entry(*c) {
          Entry::Vacant(e) => {e.insert(vec![(i as i32, j as i32)]);},
          Entry::Occupied(mut e) => {e.get_mut().push((i as i32, j as i32));},
        }
      }
    }
  }

  let mut hs = HashSet::new();
  for ant in hm.iter() {
    for (i, a) in ant.1.iter().enumerate() {
      let other = &ant.1[i..];
      for o in other {
        task1(a, o, ant.1, &mut hs, h as i32, w as i32);
      }
    }
  }
  println!("{}", hs.len());
  
  let mut hs = HashSet::new();
  for ant in hm.iter() {
    for (i, a) in ant.1.iter().enumerate() {
      let other = &ant.1[i..];
      for o in other {
        task2(a, o, ant.1, &mut hs, h as i32, w as i32);
      }
    }
  }

  let mut x: Vec<(i32, i32)> = hs.clone().into_iter().collect::<Vec<(i32,i32)>>();
  x.sort();
  println!("{:?}", x);

  println!("{}", hs.len());
}

fn task1(a: &(i32,i32), o: &(i32, i32), antennas: &[(i32, i32)], hs: &mut HashSet<(i32, i32)>, h:i32, w:i32) {
  let x = a.0 - o.0;
  let y = a.1 - o.1;
  if i32::abs(x) + i32::abs(y)>= 2 {
    match (a.0 + x, a.1 + y) {
      (i, j) 
        if i >= 0 && j >= 0 && i < h && j < w => { hs.insert((i, j)); }
      (i, j) => {println!("{:?}", (i, j));}
    }
    
    match (o.0 - x, o.1 - y) {
      (i, j) 
       if i >= 0 && j >= 0 && i < h && j < w => { hs.insert((i, j)); }
      (i, j) => {println!("{:?}", (i, j));}
    }
  }
}

fn task2(a: &(i32,i32), o: &(i32, i32), antennas: &[(i32, i32)], hs: &mut HashSet<(i32, i32)>, h:i32, w:i32) {
  let x = a.0 - o.0;
  let y = a.1 - o.1;
  hs.insert(*a);
  hs.insert(*o);

  if i32::abs(x) + i32::abs(y)>= 2 {
    let mut t = (a.0 + x, a.1 + y);
    while t.0 >= 0 && t.1 >= 0 && t.0 < h && t.1 < w {
      hs.insert(t);
      t = (t.0 + x, t.1 + y);
    } 
    
    let mut t = (o.0 - x, o.1 - y);
    while t.0 >= 0 && t.1 >= 0 && t.0 < h && t.1 < w {
      hs.insert(t);
      t = (t.0 - x, t.1 - y);
    } 
  }
}
