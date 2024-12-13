use regex::Regex;

fn main() {

  #[derive(Default, Debug)]
  struct Game {
    pub a: (u64,u64),
    pub b: (u64,u64),
    pub goal: (u64,u64) 
  }

  // Parsing

  let re = Regex::new(r"(\d+)").unwrap();
  let mut argv = std::env::args();
  let input = std::fs::read_to_string(
    format!("day13/{}", argv.nth(1).unwrap_or("test".to_string()))
    ).expect("no input file")
    .trim()
    .split("\n\n")
    .map(|s| {
      let x = s.split("\n"); 
      let mut g = Game::default();
      for d in x.enumerate() {
        match d.0 {
          0 => {
            let mut r = re.find_iter(d.1);
            g.a = (r.next().unwrap().as_str().parse::<u64>().unwrap(), r.next().unwrap().as_str().parse::<u64>().unwrap());
          },
          1 => {
            let mut r = re.find_iter(d.1);
            g.b = (r.next().unwrap().as_str().parse::<u64>().unwrap(), r.next().unwrap().as_str().parse::<u64>().unwrap());
          },
          2 => {
            let mut r = re.find_iter(d.1);
            g.goal = (r.next().unwrap().as_str().parse::<u64>().unwrap(), r.next().unwrap().as_str().parse::<u64>().unwrap());
          }
          _ => panic!()
        }
      }
      g
    })
    .collect::<Vec<Game>>();

  // Solutions

  let mut sum = 0;
  for game in &input {
    let (ax, ay) = game.a;
    let (bx, by) = game.b;
    let (x, y) = game.goal;
    let mut min = u64::MAX;
    for i in 0..=100 {
      for j in 0..=100 {
        if ax * i + bx * j == x && ay * i + by * j == y {
          min = u64::min(min, i*3+j);
        }
      }
    }
    if min != u64::MAX {
      sum += min;
    }
  }
  println!("{}", sum);

  let mut sum = 0.0;
  for game in input {
    let ax = game.a.0 as f64;
    let ay = game.a.1 as f64;
    let bx = game.b.0 as f64; 
    let by = game.b.1 as f64;
    let x = game.goal.0 as f64 + 10000000000000.0;
    let y = game.goal.1 as f64 + 10000000000000.0;

    let i = (x * by - y * bx) / (ax * by - ay * bx); 
    let j = (x - ax * i) / bx;
    if i.fract() <= f64::EPSILON && j.fract() <= f64::EPSILON {
      sum += i*3.0+j;
    }
  }
  println!("{}", sum);

}
