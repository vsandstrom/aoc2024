use std::borrow::BorrowMut;
use std::env::args;
use std::io::Split;
use std::collections::HashSet;
use std::rc::Rc;

struct Wall {
  position: (usize, usize),
}

struct Ball {
  position: (usize, usize),
}

struct Robot {
  position: (usize, usize)
}

fn main() {

  let mut input = std::fs::read_to_string(
    format!("day15/{}", args().nth(1).unwrap_or("test".to_string()))).expect("no such input");

  let mut split = input.split("\n\n");


  // let wallhs = HashSet::new();
  let mut ballhs = HashSet::new();
  let mut wallhs = HashSet::new();
  let mut robot = Robot{position: (0,0)};
  

  for (i, l) in split.next().expect("nope").lines().enumerate() {
    for (j, c) in l.chars().enumerate() {
      match c {
        'O' => {
          ballhs.insert((i, j));
          let x = Ball{position: (i, j)};
        },
        '#' => {
          wallhs.insert((i, j));
          let x = Wall{position: (i, j)};
        },
        '@' => robot = Robot{position: (i, j)},
        _ => ()
      }
    }
  }
}
