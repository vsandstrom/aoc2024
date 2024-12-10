use std::fs;
use std::env::args;

fn main() {

  let mut argv = args();
  let file = fs::read(format!("day9/{}", argv.nth(1).unwrap_or("test".to_string()))).expect("no input file");

  let mut data: Vec<Vec<u32>> = vec!();

  let mut j = 0;
  for (i, s) in file.trim_ascii_end().iter().enumerate() {
    let x = (s - b'0').to_string().parse::<usize>().expect("NaN");
    if i%2 == 0 {
      // data
      data.push(vec![j; x]); j+=1;
    } else {
      // space
      data.push(vec![0; x]);
    }
  }

  let mut hd = data.clone();
  let mut i = 1;
  let mut j = if hd.len() % 2 == 0 { hd.len() - 2 } else { hd.len() - 1 };
  let mut l: isize = (hd[j].len()-1) as isize;
  // sort
  while i < j {
    for k in 0..hd[i].len() { 
      hd[i][k] = hd[j][l as usize];
      hd[j][l as usize] = 0;
      l-=1;
      if l < 0 {
        j -= 2;
        l = (hd[j].len()-1) as isize;
      }
    }
    i+=2;
  }

  printsum(&hd);

  let mut hd = data;
  let mut j = hd.len()-1;
  let mut i = 1;
  while i < hd.len()  {
    while i < j {
      let len = hd[i].iter().filter(z1).count();
      if len >= hd[j].len() {
        let l = hd[i].iter().position(z2).unwrap();
        for k in 0..hd[j].len() {
          hd[i][l+k] = hd[j][k];
          hd[j][k] = 0;
        }
        j-=2;
      } else {
        j-=2;
      }
    }
    j = hd.len()-1;
    i+=2;
  }

  printsum(&hd);
}

fn z1(n: &&u32) -> bool {
  **n == 0
}

fn z2(n:&u32) -> bool {
  *n == 0
}

fn printsum(hd: &Vec<Vec<u32>>) {
  let mut i: u64 = 0;
  let mut sum: u64 = 0;
  for row in hd {
    for n in row {
      sum += *n as u64 * i;
      i+=1;
    }
  }
  println!("{}", sum);
}
