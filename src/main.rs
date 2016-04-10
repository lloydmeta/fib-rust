extern crate fibonacci;

use std::io;
use std::num;
use std::io::stdin;
use fibonacci::maths::fib::fib_at_index;

fn main() {
  engage_user().unwrap();
}

fn engage_user() -> Result<(), CliError> {
    println!("Which Fibonacci index are you interested in?");
    let input = try!(read_stdio());
    let index = try!(parse_positive_num(input));
    let fib_value= fib_at_index(index);
    Ok(println!("Fib at {} is {}", index, fib_value))
}

fn parse_positive_num(s: String) -> Result<i64, CliError> {
  match s.trim().parse::<i64>() {
    Ok(n)  if n >= 0 => Ok(n),
    Ok(_) => Err(CliError::IllegalArgument { description: "No negatives allowed" }),
    Err(err) => Err(CliError::Parse(err))
  }
}

fn read_stdio() -> Result<String, CliError> {
  let mut input = String::new();
  stdin().read_line(&mut input)
    .map(|_| input)
    .map_err(CliError::Io)
}

// We derive `Debug` because all types should probably derive `Debug`.
// This gives us a reasonable human readable description of `CliError` values.
#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
    IllegalArgument { description: &'static str }
}
