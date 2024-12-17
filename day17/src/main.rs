use std::usize;


struct Register{a: i32, b: i32, c: i32}

fn main() {
  let mut reg = Register{a:0,b:0,c:0};


  let mut argv = std::env::args();
  let data = std::fs::read_to_string(
    format!("day17/{}", argv .nth(1) .unwrap_or("test".to_string()))
    ).expect("no input");

  let mut data = data.lines();

  reg.a = data.next()
    .unwrap()
    .split(" ")
    .nth(2)
    .unwrap()
    .parse::<i32>()
    .unwrap();
  reg.b = 0;
  reg.c = 0;
  let program = data
    .nth(3)
    .unwrap()
    .split(" ")
    .nth(1)
    .unwrap()
    .split(",")
    .map(|n| n.parse::<i32>().unwrap())
    .collect::<Vec<_>>();

  println!("{:?}", program);

  let mut ptr = 0;
  let mut skip = false;
  while let Some(opcode) = program.get(ptr) {
    ptr+=1;
    if let Some(num) = program.get(ptr) {
      let combo = combo(*num, &reg);
      match opcode {
        0 => adv(&mut reg, combo),
        1 => bxl(&mut reg, *num),
        2 => bst(&mut reg, combo),
        3 => {skip = jnz(&mut reg, *num, &mut ptr)},
        4 => bxc(&mut reg, *num),
        5 => out(combo),
        6 => bdv(&mut reg, combo),
        7 => cdv(&mut reg, combo),
        _ => ()
      }
      if !skip {
        ptr+=1;
      }
      skip = false;
    }
  }
}

fn combo(num: i32, reg: &Register) -> i32 {
  match num {
    0..=3 => num,
    4 => reg.a,
    5 => reg.b,
    6 => reg.c,
    7 => 0,
    _ => panic!("not 3 bit number")
  }
}

fn adv(reg: &mut Register, b: i32) { reg.a /= i32::pow(2, b.try_into().unwrap()) }

fn bxl(reg: &mut Register, a: i32) { reg.b ^= a }
fn bst(reg: &mut Register, b: i32) { reg.b = b%8 }
fn jnz(reg: &mut Register, a: i32, ptr: &mut usize) -> bool { if reg.a != 0 { *ptr = a as usize; true } else { false }}
fn bxc(reg: &mut Register,_a: i32) { reg.b^=reg.c }
fn out(b: i32) { print!("{},", b%8) }
fn bdv(reg: &mut Register, b: i32) { reg.b = reg.a / i32::pow(2, b.try_into().unwrap()) }
fn cdv(reg: &mut Register, b: i32) { reg.c = reg.a / i32::pow(2, b.try_into().unwrap()) }
