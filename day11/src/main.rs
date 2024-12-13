use std::collections::HashMap;
use std::fs;
use std::env::args;

fn main() {
  let mut argv = args();
  let input = fs::read_to_string(format!("day11/{}", argv.nth(1).unwrap_or("test".to_string())))
    .expect("no input file")
    .trim()
    .split(" ")
    .map(|s| s.to_string().parse::<u64>().expect("unable to parse"))
    .collect::<Vec<_>>();

  let mut stones = input.clone();

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

  println!("{}", stones.len());
  let mut hm = HashMap::new();
  let mut sum = 0;
  for num in stones.iter() {
    let mut lvl = 0;
    sum += traverse(&mut lvl, num, &mut hm);
  }

  println!("{}", sum);
}

fn split_num(num: u64) -> (u64, u64) {
  let x = num.ilog10()+1;
  let left = num / u64::pow(10, x/2);
  let right = num - left * u64::pow(10, x/2);
  (left, right)
}

fn traverse(i: &mut usize, num: &u64, hm: &mut HashMap<(usize, u64), usize>) -> usize {
  if let Some (val) = hm.get(&(*i, *num)) {
    return *val
  }
  if *i == 75 { return 1 }
  match num {
    n if *n == 0 => {
      *i += traverse(i, &1, hm);
      *i
    },
    n if u64::ilog10(*n)%2 == 1 => {
      let (a, b) = split_num(*n);
      *i += traverse(i, &a, hm);
      *i += traverse(i, &b, hm);
      *i
    },
    n => {
      *i += traverse(i, &(*n * 2024), hm);
      *i
    }
  }
}

