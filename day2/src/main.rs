use std::fs;

fn main() {
  let data = fs::read_to_string("day2/input")
    .expect("file parsing")
    .lines()
    .map(|s| 
      s.split(" ")
        .map(|n| 
          n.parse::<i32>()
          .unwrap()
        ).collect::<Vec<i32>>()
    ).collect::<Vec<Vec<i32>>>();

  let dir = |x: i32, y: i32| {
    match (x, y) {
      (x, y) if x < y => Some(true),
      (x, y) if x > y => Some(false),
      (x, y) if x == y => None,
      _ => panic!()
    }
  };

  let mut c = data.len();
  for row in &data {
    let mut p = None;
    'inner: for win in row.windows(2) {
      match p {
        // uninitialized p
        None => { 
          let d = dir(win[0], win[1]);
          // if first pair is equal:
          if d.is_none() { c-=1; break 'inner }
          if i32::abs(win[0] - win[1]) > 3 { c-=1; break 'inner }
          p = d;
        },
        Some(d) => {
          match dir(win[0], win[1]) {
            // either inc or dec
            Some(w) => {
              // 
              if w == d && i32::abs(win[0] - win[1]) <= 3 { continue 'inner; }
              c-=1; 
              break 'inner
            },
            None => {
              c-=1; break 'inner
            }
          }
        }
      }
    }
  }
  println!("{}", c);

}
