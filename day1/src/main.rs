use std::fs;

fn main() {

  // ID and LOCATION in list, key: ID, value: location
  let mut v1 = vec!();
  let mut v2 = vec!();

  for x in fs::read_to_string("../input").expect("file parsing").lines() {
    let mut row = x.trim().split("   ");
    let s1 = row.next().unwrap().parse::<i32>().unwrap();
    let s2 = row.last().unwrap().parse::<i32>().unwrap();
    v1.push(s1);
    v2.push(s2);
  }

  v1.sort();
  v2.sort();
  let mut sum= 0;

  for (x, y) in v1.iter().zip(v2.iter()) {
    sum += i32::abs(x - y);
  }

  let mut sum2 = 0;
  for e in v1 {
    for x in &v2 {
      if *x == e {
        sum2 += *x;
      }
    }
  }

  println!("{}", sum);
  println!("{}", sum2);
}
