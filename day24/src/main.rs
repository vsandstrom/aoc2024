use std::{borrow::BorrowMut, collections::HashMap, hash::Hash};

fn main() {
  let mut argv = std::env::args();
  let data = std::fs::read_to_string(
    format!("day24/{}", argv.borrow_mut().nth(1).clone().unwrap_or("test".to_string())))
    .expect("no input file");

  let mut values: HashMap<String, Option<bool>> = HashMap::new();


  let mut ds = data.split("\n\n");
  for init in ds.next().unwrap().lines() {
    // println!("{:?}", init);
    let mut s = init.split(": ");
    // println!("{:?}", s.map(|s| s.to_string()).collect::<Vec<String>>());
    values.insert(
      s.next().unwrap().to_string(),
      if s.next().unwrap() == "1" { Some(true) } else {Some(false)}
    );
  }

  let mut top_z = String::new();
  let mut top_z_num = 0;

  let operations = ds.next().unwrap();
  let mut wires = vec!();
  for op in operations.lines() {
    let ws = op.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
    if ws[0].starts_with("z") {
      let num = ws[0][1..].parse::<i32>().unwrap();
      if num > top_z_num {
        top_z_num = num;
        top_z = ws[0].clone();
      }
    }
    if ws[2].starts_with("z") {
      let num = ws[2][1..].parse::<i32>().unwrap();
      if num > top_z_num {
        top_z_num = num;
        top_z = ws[2].clone();
      }
    }
    if ws[4].starts_with("z") {
      let num = ws[4][1..].parse::<i32>().unwrap();
      if num > top_z_num {
        top_z_num = num;
        top_z = ws[4].clone();
      }
    }

    values.entry(ws[0].clone()).or_insert(None);
    values.entry(ws[2].clone()).or_insert(None);
    values.entry(ws[4].clone()).or_insert(None);
    wires.push(ws);
  }

  // for x in values.iter() {
  //   if x.0.starts_with("z") { println!("{:?}", x) }
  // }


  'outer: loop {

    for w in wires.iter() {
      let a = values.get(&w[0]).unwrap();
      if a.is_none() { continue; }
      let b = values.get(&w[2]).unwrap();
      if b.is_none() { continue; }
      let c = values.get(&w[4]).unwrap();
      if a.is_some() && b.is_some() && c.is_none() {
        let mut x: Option<bool> = None;
        match w[1].as_str() {
          "AND" => x = Some(a.unwrap() & b.unwrap()), 
          "OR" =>  x = Some(a.unwrap() | b.unwrap()),
          "XOR" => x = Some(a.unwrap() ^ b.unwrap()),
          _=> ()

        }
        values.entry(w[4].clone()).and_modify(|y| *y = x); 
      }
    }
    if values.iter().filter(|z| z.0.starts_with("z") && z.1.is_none()).count() == 0 {
      break 'outer;
    }
  }

  let mut z_vec: Vec<(&String, &Option<bool>)> = values
    .iter()
    .filter(|z| z.0.starts_with("z"))
    .collect();
  let mut x_vec: Vec<(&String, &Option<bool>)> = values
    .iter()
    .filter(|x| x.0.starts_with("x"))
    .collect();
  let mut y_vec: Vec<(&String, &Option<bool>)> = values
    .iter()
    .filter(|y| y.0.starts_with("y"))
    .collect();
  z_vec.sort();
  z_vec.reverse();
  x_vec.sort();
  x_vec.reverse();
  y_vec.sort();
  y_vec.reverse();
  let mut s = vec!();
  for a in &z_vec {
    if a.1.unwrap() { s.push('1'); } 
    else { s.push('0'); }
  }
  let zn = isize::from_str_radix(s.into_iter().collect::<String>().as_str(), 2).unwrap();
  println!("z: {}", zn);
  // let mut s = vec!();
  // for a in &x_vec {
  //   if a.1.unwrap() { s.push('1'); } 
  //   else { s.push('0'); }
  // }
  // let xn = isize::from_str_radix(s.into_iter().collect::<String>().as_str(), 2).unwrap();
  // println!("x: {}", xn);
  // let mut s = vec!();
  // for a in &y_vec {
  //   if a.1.unwrap() { s.push('1'); } 
  //   else { s.push('0'); }
  // }
  // let yn = isize::from_str_radix(s.into_iter().collect::<String>().as_str(), 2).unwrap();
  // println!("y: {}", yn);

  // println!("{:08b}", (xn & 63) & (yn & 63));
  // println!("{:08b}", ((xn & 63) & (yn & 63)) | zn & 63);

  // for w in wires {
  //   if w[1] == "AND" {
  //     println!("{:?}", w);
  //   }
  // }
}

