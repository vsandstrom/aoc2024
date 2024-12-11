use std::fs;
use std::env::args;

fn main() {
  let mut argv = args();
  let mut stones = fs::read_to_string(format!("day11/{}", argv.nth(1).unwrap_or("test".to_string())))
    .expect("no input file")
    .trim()
    .split(" ")
    .map(|s| s.to_string().parse::<u64>().expect("unable to parse"))
    .collect::<Vec<_>>();
  for _ in 0..25 {
    let mut temp = vec!();
    for num in stones.iter() {
      match num {
        n if *n == 0 => {
          temp.push(1);
        },
        n if u64::ilog10(*n)%2 == 1 => {
          let (a, b) = split_num(*n);
          temp.push(a);
          temp.push(b);
        },
        n => {
          temp.push(n * 2024);
        }
      }
    }
    stones = temp;
  }

  println!("{}", stones.len())
}

fn split_num(num: u64) -> (u64, u64) {
  let x = num.ilog10()+1;
  let left = num / u64::pow(10, x/2);
  let right = num - left * u64::pow(10, x/2);
  (left, right)
}
