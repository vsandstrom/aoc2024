use std::fs;

fn main() {
  let mut v1 = vec!();
  let mut v2 = vec!();

  for x in fs::read_to_string("day1/input").expect("file parsing").lines() {
    let mut row = x.trim().split("   ");
    let s1 = row.next().unwrap().parse::<i32>().unwrap();
    let s2 = row.last().unwrap().parse::<i32>().unwrap();
    v1.push(s1);
    v2.push(s2);
  }

  v1.sort();
  v2.sort();

  let sum = v1.iter().zip(v2.iter()).fold(0, |acc, (x, y)| { acc + i32::abs(x-y) });

  let mut sum2 = 0;
  for e in v1 {
    let mut found = false;
    'inner: for x in &v2 {
      if *x == e {
        found = true;
        sum2 += *x;
      } else if found && *x != e {
        // break early
        break 'inner;
      }
    }
  }

  println!("{}", sum);
  println!("{}", sum2);
}
