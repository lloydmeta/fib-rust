extern crate fibonacci;

use std::num::ParseIntError;
use std::io;
use fibonacci::maths::fib::fib_at_index;

fn main() {
    println!("Which Fibonacci index are you interested in?");
    let _ = parse_positive_num(read_stdio()).map ( |i| {
      let fib_value= fib_at_index(i);
      println!("Fib at {} is {}", i, fib_value)
    });
}

fn parse_positive_num(s: String) -> Result<i64, ParseIntError> {
  match s.trim().parse::<i64>() {
    Ok(n)  if n >= 0 => Ok(n),
    Ok(_) => panic!("no negatives allowed"),
    Err(err) => Err(err)
  }
}

fn read_stdio() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  input
}
