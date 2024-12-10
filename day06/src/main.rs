use std::{collections::HashSet, fs};

#[derive(Clone, Copy, Default, Debug, Hash, PartialEq, Eq)]
enum Dir {
  #[default]
  Up,
  Down,
  Right,
  Left
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Guard {
  pub pos: (i32, i32),
  pub dir: Dir
}

impl Guard {
  fn traverse(&self) -> (i32, i32) {
    match self.dir {
      Dir::Up =>    (self.pos.0 - 1, self.pos.1),
      Dir::Down =>  (self.pos.0 + 1, self.pos.1),
      Dir::Right => (self.pos.0, self.pos.1 + 1),
      Dir::Left =>  (self.pos.0, self.pos.1 - 1)
    }
  }

  fn traverse_with_pos(&self) -> Self {
    match self.dir {
      Dir::Up =>    Self{pos:(self.pos.0 - 1, self.pos.1), dir: self.dir},
      Dir::Down =>  Self{pos:(self.pos.0 + 1, self.pos.1), dir: self.dir},
      Dir::Right => Self{pos:(self.pos.0, self.pos.1 + 1), dir: self.dir},
      Dir::Left =>  Self{pos:(self.pos.0, self.pos.1 - 1), dir: self.dir}
    }
  }

  fn rotate(&mut self) {
    match self.dir {
      Dir::Up => self.dir = Dir::Right,
      Dir::Down => self.dir = Dir::Left,
      Dir::Right => self.dir = Dir::Down,
      Dir::Left => self.dir = Dir::Up
    }
  }
}

fn main() {
  let mut pos: HashSet<(i32, i32)> = HashSet::new();
  let mut startposition = Guard{pos: (0,0), dir: Dir::Up};
  let mut walk: HashSet<(i32, i32)> = HashSet::new();
  // let height = 10;
  // let width = 10;
  let height = 130;
  let width = 130;
  for (i, l) in fs::read_to_string("day06/input").expect("no input").lines().enumerate() {
    for (j,c) in l.chars().enumerate() {
      if c == '#' {
        pos.insert((i.try_into().expect("unconvertable"), j.try_into().expect("unconvertable")));
      }
      if c == '^' {
        startposition.pos = (i.try_into().expect("unconvertable"), j.try_into().expect("unconvertable"));
      }
    }
  };

  let mut guard = startposition;
  while guard.pos.0 >= 0 && guard.pos.0 < height && guard.pos.1 >= 0 && guard.pos.1 < width {
    walk.insert(guard.pos);
    // potential position
    let p = guard.traverse();
    if pos.contains(&p) {
    // if obsticle, rotate
      guard.rotate();
    } else {
    // else move to potential position, update walked tiles
      guard.pos = p;
    }
  }
  println!("{}", walk.len());


  let mut opts = 0;
  // choose position to be blocked. 
  for i in 0..height {
    for j in 0..width {
      // new run:
      let mut guard = startposition;
      let mut walk_with_pos: HashSet<Guard> = HashSet::new();
      // while inside room
      while guard.pos.0 >= 0 && guard.pos.0 < height && guard.pos.1 >= 0 && guard.pos.1 < width {
        // if we are treading in our own steps
        if walk_with_pos.contains(&guard) {
          opts+=1;
          break;
        } 

        walk_with_pos.insert(guard);

        // potential position
        let p = guard.traverse_with_pos();
        if pos.contains(&p.pos) || p.pos == (i,j) {
        // if obsticle or placed object, rotate
          guard.rotate();
        } else {
        // else move to potential position, update walked tiles
          guard = p;

        }
      }

    }
  }

  println!("{}", opts);
}
