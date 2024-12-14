use std::{fs, usize};
use std::collections::{HashMap, HashSet, hash_map::Entry};
use std::cmp::Ordering;


fn main() {
  let file = fs::read_to_string("day05/input").expect("file parsing");
  let mut data = file.split("\n\n");
  let mut bck_rules: HashMap<i32, HashSet<i32>> = HashMap::new();
  let mut fwd_rules: HashMap<i32, HashSet<i32>> = HashMap::new();
  let rule_data = data.next().unwrap().lines();
  for row in rule_data {
    let parsed_row = row
      .split("|")
      .map(|r| r.parse::<i32>().expect("unable to parse"))
      .collect::<Vec<i32>>();

    let rule = parsed_row;
    match fwd_rules.entry(rule[0]) {
      Entry::Vacant(e) => {
        let mut hs = HashSet::new();
        hs.insert(rule[1]);
        e.insert(hs);
      },
      Entry::Occupied(mut e) =>  {
        e.get_mut().insert(rule[1]);
      }
    }
    
    match bck_rules.entry(rule[1]) {
      Entry::Vacant(e) => {
        let mut hs = HashSet::new();
        hs.insert(rule[0]);
        e.insert(hs);
      },
      Entry::Occupied(mut e) =>  {
        e.get_mut().insert(rule[0]);
      }
    }
  }

  let mut  filtered_orders = vec!();

  let mut sum = 0;
  'outer: for row in data.next().unwrap().lines() {
    let page_order = row.split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    for (i, page) in page_order.iter().enumerate() {
      if let Some(rule) = fwd_rules.get(page) {
        let order = &page_order[(i+1)..];
        if !order.is_empty() {
          for p in order {
            if !rule.contains(p) {
              filtered_orders.push(page_order);
              continue 'outer;
            }
          }
        }
      }
      if let Some(rule) = bck_rules.get(page) {
        let order = &page_order[..i];
        if !order.is_empty() {
          for p in order {
            if !rule.contains(p) {
              filtered_orders.push(page_order);
              continue 'outer;
            }
          }
        }
      }
    }
    let j = page_order.len() / 2;
    sum += page_order[j];
  }
  println!("{}", sum);

  let mut sum = 0;
  for mut page_order in filtered_orders { 
    page_order.sort_by(|a, b| {
      match fwd_rules.get(a) {
        Some(hs) => { 
          if hs.contains(b) {Ordering::Less }
          else {Ordering::Greater}
        },
        None => Ordering::Equal
    }});

    let j = page_order.len() / 2;
    sum += page_order[j];
  }
  println!("{}", sum);
}
