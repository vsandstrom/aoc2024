use std::env::args;

fn main() {

  let mut input = std::fs::read_to_string(
    format!("day15/{}", args().nth(1).unwrap_or("test".to_string()))).expect("no such input");

  let (mut room, moves) = input
    .split("\n\n")
    .collect::<Vec<&str>>()[..]
    .windows(2)
    .next()
    .map(|x| {
    (
      x[0]
      .lines()
      .map(|l| 
        l.chars()
        .collect()
      ).collect::<Vec<Vec<char>>>(), 
       x[1]
       .lines()
       .fold(String::new(), |mut x, c| {
         x = format!("{x}{c}");
         x
        
       })
       .chars()
       .collect::<Vec<char>>()
     )
  }).unwrap();

  // room.iter().for_each(|l| { println!("{:?}", l) });

  // find robot
  let mut robot = (0, 0);
  'search: for (i, l) in room.iter().enumerate() {
    if let Some(r) = l.iter().position(|x| *x=='@') {
     robot = (i, r);
     break 'search;
    }
  }

  for m in moves {
    match m {
      'v' => { 
        if let Some(x) = move_robot(robot, &mut room, |mut pos| {pos.0 += 1; pos}) { robot = x; };
      }, // ner
      '^' => { 
        if let Some(x) = move_robot(robot, &mut room, |mut pos| {pos.0 -=1; pos}) { robot = x; };
      }, // upp
      '<' => { 
        if let Some(x) = move_robot(robot, &mut room, |mut pos| {pos.1 -=1; pos}) { robot = x; };
      }, // vänster
      '>' => { 
        if let Some(x) = move_robot(robot, &mut room, |mut pos| {pos.1 +=1; pos}) { robot = x; };
      }, // höger
      _ => { unreachable!() }
    }
    // println!("{m}");
    // room.iter().for_each(|l| println!("{}", l[..].iter().collect::<String>()));
    // println!()
  }

  let mut total_sum = 0;
  for (x, row) in room.iter().enumerate() {
    for (y, c) in row.iter().enumerate() {
      if *c == 'O' {
        total_sum += (x*100) + y;
      }
    }
  }

  println!("{total_sum}");

}

fn move_robot<F: FnMut((usize, usize))-> (usize, usize)>(robot: (usize, usize), room: &mut [Vec<char>], mut m: F) -> Option<(usize, usize)> {
  let mut next_pos = m(robot);
  let mut box_stack = Vec::new();
  loop {
    match room[next_pos.0][next_pos.1] {
      '.' => break,
      '#' => return None,
      'O' => {
        box_stack.push(next_pos);
        next_pos = m(next_pos);
      },
      '@' => { println!("oops") }
      _ => unreachable!()
    }
  }

  while let Some(x) = box_stack.pop() {
    let y = m(x);
    room[y.0][y.1] = 'O';
  }

  room[robot.0][robot.1] = '.';
  let x = m(robot);
  room[x.0][x.1] = '@';
  Some(x)
}
