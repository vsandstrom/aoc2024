use std::time::Duration;
use std::thread::sleep;

fn main() {
  let mut argv = std::env::args();
  let path = argv.nth(1);


  let data = std::fs::read_to_string(
    format!(
      "day14/{}", 
      path.clone().unwrap_or("test".to_string())))
    .expect("no input")
    .lines()
    .map(|l| {
      l.split(' ')
      .map(|s| {
        let x = s[2..].to_string();
        let mut xs = x.split(',');
        (
          xs.next().unwrap().parse::<i32>().unwrap(), 
         xs.next().unwrap().parse::<i32>().unwrap()
        )
      })
      .collect::<Vec<(i32, i32)>>()})
    .collect::<Vec<Vec<(i32, i32)>>>();
    let (h, w) = match path {
      Some(_) => {(103, 101)},
      None    => {( 11,   7)}
    };

    let mut quads = [0;4];
    for robot in &data {
      let mut pos = robot[0];
      let vel = robot[1];
      let vel100 = (vel.0*100, vel.1*100);
      pos.0 += vel100.0;
      pos.1 += vel100.1;
      while pos.0 >= w { pos.0 -= w; }
      while pos.0 <  0 { pos.0 += w; }
      while pos.1 >= h { pos.1 -= h; }
      while pos.1 <  0 { pos.1 += h; }

      match pos {
        (x, y) if x < w/2 && y < h/2 => quads[0]+=1,
        (x, y) if x > w/2 && y < h/2 => quads[1]+=1,
        (x, y) if x < w/2 && y > h/2 => quads[2]+=1,
        (x, y) if x > w/2 && y > h/2 => quads[3]+=1,
        _ => ()
        // (x, y) => println!("{:?}", (x,y))
      }
    }
    
    println!("{}", quads.iter().product::<i32>());

    let mut data = data;
    let mut j: u128 = 0;

    'finder: loop {
      let mut space = vec![vec![' '; h as usize]; w as usize];
      for robot in data.iter() {
        space[robot[0].0 as usize][robot[0].1 as usize] = '#';
      }

      for window in space.windows(3) {
        for i in 0..(w as usize - 3) {
          if ['#', '#', '#', '#', '#', '#', '#', '#', '#'] == [
            window[0][i], window[0][i+1], window[0][i+2],
            window[1][i], window[1][i+1], window[1][i+2],
            window[2][i], window[2][i+1], window[2][i+2]
          ] {
            break 'finder;
          }
        }
      }


      for robot in data.iter_mut() {
        let mut pos = robot[0];
        let vel = robot[1];
        pos.0 += vel.0;
        pos.1 += vel.1;
        while pos.0 >= w { pos.0 -= w; }
        while pos.0 <  0 { pos.0 += w; }
        while pos.1 >= h { pos.1 -= h; }
        while pos.1 <  0 { pos.1 += h; }
        robot[0] = pos;
      }

      j+=1;
    }
    println!("{}", j);
}

