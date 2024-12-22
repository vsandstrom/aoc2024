use std::collections::HashSet;

// const SIZE: i32 = 6;
// const BYTES: usize = 12;
const SIZE: i32 = 70;
const BYTES: usize = 1024;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Byte (i32, i32);
  
fn main() {
  let mut argv = std::env::args();
  let bytes = std::fs::read_to_string(format!("day18/{}", argv.nth(1).unwrap_or("test".into())))
    .expect("no input found")
    .lines()
    .map(|l| {
      let mut x = l.split(",");
      Byte(x.next().unwrap().parse().unwrap(), x.next().unwrap().parse().unwrap())
    }).collect::<Vec<Byte>>();

  // insert the first 1024 obstacles
  let mut hs = HashSet::new();
  for b in bytes.iter().take(BYTES) {
    hs.insert(b);
  }

  let mut frontier = vec![Byte(0,0)];
  let mut next_frontier = vec!();
  let mut traversed: HashSet<Byte> = HashSet::from([Byte(0,0)]);
  traversed.extend(hs.clone());
  let mut c = 0;
  'outer: loop {
    while let Some(b) = frontier.pop() {
      if b == Byte(SIZE,SIZE) {
        break 'outer;
      }
      if 0 < b.0 && !traversed.contains(&Byte(b.0 - 1, b.1)) {
        let b2 = Byte(b.0 - 1, b.1);
        next_frontier.push(b2);
        traversed.insert(b2);
      }

      if b.0 < SIZE && !traversed.contains(&Byte(b.0 + 1, b.1)) {
        let b2 = Byte(b.0 + 1, b.1);
        next_frontier.push(b2);
        traversed.insert(b2);
      }

      if b.1 < SIZE && !traversed.contains(&Byte(b.0, b.1 + 1)) {
        let b2 = Byte(b.0, b.1 +1);
        next_frontier.push(b2);
        traversed.insert(b2);
      }
        
      if 0 < b.1 && !traversed.contains(&Byte(b.0, b.1 - 1)) { 
        let b2 = Byte(b.0, b.1 - 1);
        next_frontier.push(b2);
        traversed.insert(b2);
      } 
    }
    if frontier.is_empty() && next_frontier.is_empty() { break; }
    frontier = next_frontier.clone();
    c+=1;
    next_frontier.clear();
  }
  println!("{}", c);

  'outer_outer: for byte in &bytes[1024..] {
    hs.insert(byte);
    let mut frontier = vec![Byte(0,0)];
    let mut next_frontier = vec!();
    let mut traversed: HashSet<Byte> = HashSet::from([Byte(0,0)]);
    traversed.extend(hs.clone());
    'outer: loop {
      while let Some(b) = frontier.pop() {
        if b == Byte(SIZE,SIZE) {
          break 'outer;
        }
        if 0 < b.0 && !traversed.contains(&Byte(b.0 - 1, b.1)) {
          let b2 = Byte(b.0 - 1, b.1);
          next_frontier.push(b2);
          traversed.insert(b2);
        }

        if b.0 < SIZE && !traversed.contains(&Byte(b.0 + 1, b.1)) {
          let b2 = Byte(b.0 + 1, b.1);
          next_frontier.push(b2);
          traversed.insert(b2);
        }

        if b.1 < SIZE && !traversed.contains(&Byte(b.0, b.1 + 1)) {
          let b2 = Byte(b.0, b.1 +1);
          next_frontier.push(b2);
          traversed.insert(b2);
        }
          
        if 0 < b.1 && !traversed.contains(&Byte(b.0, b.1 - 1)) { 
          let b2 = Byte(b.0, b.1 - 1);
          next_frontier.push(b2);
          traversed.insert(b2);
        } 
      }
      if frontier.is_empty() && next_frontier.is_empty() { 
        println!("{:?}", byte); break 'outer_outer;
      }
      frontier = next_frontier.clone();
      c+=1;
      next_frontier.clear();
    } 
  }
}
