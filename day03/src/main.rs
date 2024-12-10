use std::fs;
use regex::Regex;
fn main() {
  let re1 = Regex::new(r"mul\(\d+,\d+\)").unwrap();
  let re2 = Regex::new(r"(mul\(\d+,\d+\)|don\'t\(\)|do\(\))").unwrap();
  let data = fs::read_to_string("day3/input").expect("file parsing");
  let mut sum = 0;
  for instr in re1.find_iter(&data).map(|m|m.as_str()) {
    sum += mul(instr);
  }
  println!("{}", sum);

  let mut sum = 0;
  let mut d = true;
  for instr in re2.find_iter(&data).map(|m|m.as_str()) {
    match instr {
      "don't()" => d = false,
      "do()"    => d = true,
      _ => { if d { sum+=mul(instr); } }
    }
  }
  println!("{}", sum);
}

fn mul(instr: &str) -> i32 {
  let m = instr.split(&['(',',',')']).collect::<Vec<&str>>();
  m[1].parse::<i32>().unwrap() * m[2].parse::<i32>().unwrap()
}
